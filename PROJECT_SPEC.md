# InterpretableChessEngine - Project Specification

> **This project explores whether patterns discovered through graph-theoretic analysis can be translated into highly abstract, human-interpretable strategic concepts—enabling a universal chess engine that explains its reasoning in terms humans can understand and apply across any variant.**

---

## 1. Executive Summary

### Project Name
**InterpretableChessEngine**

Repository: https://github.com/Gingnose/InterpretableChessEngine

### Core Question
**いかにシンプルな評価関数で強くすることができるか？**

シンプルな評価関数は：
- 人間による解釈が可能
- バリアントへの汎化が容易
- デバッグと改善が明確

### Success Definition
1. 標準チェスで合理的な強さを達成
2. 評価の「理由」を人間が理解できる形で出力
3. 新しい駒（Amazon, Camel）を追加した際、手動調整なしで有効活用できる

---

## 2. Vision & Goals

### 2.1 解釈可能なチェスエンジン
従来のエンジン（Stockfish NNUE等）はブラックボックス。本プロジェクトは：
- なぜその手が良いのかを説明できる
- 評価の内訳を人間が理解できる形で表示
- 戦術（フォーク、ピン等）を明示的に検出・報告

### 2.2 グラフ理論による汎用的評価
- 駒の「名前」ではなく「グラフ上の性質」で評価
- 中心性、連結性、脅威関係から価値を導出
- 新しい駒でも自動的に適切な評価が行われる

### 2.3 バリアントへの汎化
目標：駒の「動き方」だけを定義すれば、評価関数は自動的に適応

テストケース：
- **Amazon**（Queen + Knight の動き）: 強力な駒として有効活用できるか
- **Camel**（1,3 のリーパー）: 新しい動きでフォーク・チェックメイトを発見できるか

---

## 3. Technical Stack

### 言語
**Rust**
- ゼロコスト抽象化
- メモリ安全性
- 高いパフォーマンス
- 優れた型システム

### 手法
| 要素 | 技術 |
|------|------|
| 動きの定義 | 整数ベクトル + 対称性生成関数 |
| 脅威検出 | レイキャスト / Bitboard |
| 戦術分類 | 脅威集合のパターンマッチング |
| 評価 | グラフ特徴量（中心性、連結性等） |
| 探索 | Alpha-Beta / MCTS |

---

## 4. Core Concepts

### 4.1 Threat（脅威）の定義

脅威は評価の基本単位。駒ではなく「脅威」で考える。

```rust
/// 脅威の抽象的定義
struct Threat {
    /// 脅威の起点（攻撃者の位置）
    origin: Square,
    /// 脅威のベクトル（方向 + 距離）
    vector: Vector2,
    /// 到達した際の報酬
    reward: ThreatReward,
}

/// 報酬の種類
enum ThreatReward {
    /// 駒の捕獲
    Capture { piece_value: i32 },
    /// チェック
    Check,
    /// チェックメイト
    Checkmate,
    /// ピンの可能性（後ろにより価値の高い駒）
    Pin { pinned_value: i32, behind_value: i32 },
    /// ポジショナルな利得
    Positional { score: f32 },
    /// 複合報酬（フォーク: Capture + Check など）
    Multiple(Vec<ThreatReward>),
}

impl Threat {
    /// 到達点を計算
    fn target(&self) -> Square {
        self.origin + self.vector
    }
    
    /// 正規化された方向
    fn direction(&self) -> Vector2 {
        self.vector.normalize()
    }
}
```

### 4.2 戦術の統一的理解

**核心的洞察**：ピンとフォークは本質的に同じ現象の異なる表現

```
フォーク = 一手で複数の脅威を生成（異なる方向）
ピン    = 一手で複数の脅威を生成（同一直線上、弱い駒が前）

本質：「一手で複数の脅威を作る」
```

```rust
/// 一手から生じる脅威の集合
struct MoveThreats {
    move_: Move,
    threats: Vec<Threat>,
}

/// 戦術パターンの分類
enum TacticType {
    /// 単一の脅威
    SingleThreat,
    /// 直線上の複数脅威（ピン、スキュアー）
    LinearMultiThreat,
    /// 非直線の複数脅威（フォーク）
    DivergentMultiThreat,
    /// チェック + 捕獲（特に価値が高い）
    CheckWithCapture,
}

fn classify_tactic(move_threats: &MoveThreats) -> TacticType {
    let threats = &move_threats.threats;
    
    if threats.len() < 2 {
        return TacticType::SingleThreat;
    }
    
    // 全ての脅威が同一直線上か？
    let directions: Vec<_> = threats.iter()
        .map(|t| t.direction())
        .collect();
    
    if all_collinear(&directions) {
        TacticType::LinearMultiThreat
    } else {
        TacticType::DivergentMultiThreat
    }
}
```

### 4.3 グラフ理論的評価

盤面をグラフとして分析し、戦略的概念を導出。

#### ノードとエッジ

```rust
/// グラフのノード
enum Node {
    Piece(Piece),    // 駒
    Square(Square),  // マス
}

/// グラフのエッジ
struct Edge {
    from: NodeId,
    to: NodeId,
    edge_type: EdgeType,
    direction: Vector2,  // 方向情報を保持
}

enum EdgeType {
    Attacks,     // 攻撃関係
    Defends,     // 防御関係
    CanReach,    // 到達可能
    Occupies,    // 占有
}
```

#### グラフ特徴量

| 特徴量 | 定義 | チェスでの意味 |
|--------|------|---------------|
| 次数中心性 | ノードに接続するエッジ数 | 機動力、影響力 |
| 媒介中心性 | 最短経路上に存在する頻度 | 争点となるマス/駒 |
| 連結成分 | 互いに到達可能なノード群 | 駒の協調、孤立 |
| クラスタ係数 | 隣接ノード同士の連結度 | 陣形の堅固さ |

### 4.4 汎化の原則

**鉄則：駒の「名前」を評価に使わない**

```rust
// ❌ 汎化不可能（駒名に依存）
fn evaluate_piece_bad(piece: &Piece) -> i32 {
    match piece.piece_type {
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Amazon => ???,  // 新しい駒は定義できない
    }
}

// ✅ 汎化可能（グラフ性質に依存）
fn evaluate_piece_good(piece: &Piece, graph: &PositionGraph) -> i32 {
    let mobility = graph.reachable_squares(piece).len();
    let centrality = graph.centrality(piece);
    let threats = graph.threats_from(piece).len();
    let defended = graph.defended_by(piece).len();
    
    // 駒名を使わない → 任意の駒に適用可能
    (W_MOBILITY * mobility +
     W_CENTRALITY * centrality +
     W_THREATS * threats +
     W_DEFENDED * defended) as i32
}
```

---

## 5. Technical Architecture

### 5.1 モジュール構成

```
interpretable-chess-engine/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   │
│   ├── core/                    # コアデータ構造
│   │   ├── mod.rs
│   │   ├── square.rs           # マス表現
│   │   ├── piece.rs            # 駒定義
│   │   ├── board.rs            # 盤面状態
│   │   ├── movement.rs         # 動きの定義
│   │   └── game_state.rs       # ゲーム状態
│   │
│   ├── movegen/                 # 動き生成
│   │   ├── mod.rs
│   │   ├── bitboard.rs         # Bitboard 実装
│   │   ├── rays.rs             # レイキャスト
│   │   └── legal_moves.rs      # 合法手生成
│   │
│   ├── threats/                 # 脅威分析
│   │   ├── mod.rs
│   │   ├── threat.rs           # Threat 構造体
│   │   ├── detection.rs        # 脅威検出
│   │   └── tactics.rs          # 戦術分類
│   │
│   ├── graph/                   # グラフ理論
│   │   ├── mod.rs
│   │   ├── position_graph.rs   # 局面グラフ
│   │   ├── centrality.rs       # 中心性計算
│   │   ├── connectivity.rs     # 連結性分析
│   │   └── features.rs         # 特徴量抽出
│   │
│   ├── eval/                    # 評価関数
│   │   ├── mod.rs
│   │   ├── evaluator.rs        # 評価器トレイト
│   │   ├── graph_eval.rs       # グラフベース評価
│   │   ├── threat_eval.rs      # 脅威ベース評価
│   │   └── explanation.rs      # 評価の説明生成
│   │
│   ├── search/                  # 探索
│   │   ├── mod.rs
│   │   ├── alpha_beta.rs       # Alpha-Beta 探索
│   │   ├── mcts.rs             # MCTS（オプション）
│   │   └── transposition.rs    # 置換表
│   │
│   ├── variants/                # バリアント定義
│   │   ├── mod.rs
│   │   ├── standard.rs         # 標準チェス
│   │   ├── amazon.rs           # Amazon バリアント
│   │   └── camel.rs            # Camel バリアント
│   │
│   └── uci/                     # UCI プロトコル
│       ├── mod.rs
│       └── protocol.rs
│
├── benches/                     # ベンチマーク
│   └── search_bench.rs
│
└── tests/                       # 統合テスト
    ├── tactics_test.rs
    └── variant_test.rs
```

### 5.2 データ構造

#### 駒と動きの定義

```rust
/// 動きのタイプ
enum MovementType {
    /// スライド移動（ルーク、ビショップ）
    Slide {
        directions: Vec<Vector2>,
        max_distance: Option<u8>,
        blocked_by_pieces: bool,
    },
    /// リープ移動（ナイト）
    Leap {
        offsets: Vec<Vector2>,
    },
    /// ポーンの特殊移動
    Pawn {
        forward: Vector2,
        capture_directions: Vec<Vector2>,
        double_move_from_rank: Option<u8>,
    },
}

/// 駒の定義
struct PieceDefinition {
    /// 駒の名前（表示用のみ、評価には使わない）
    name: String,
    /// 動きのリスト
    movements: Vec<MovementType>,
    /// 王かどうか（勝利条件に関係）
    is_royal: bool,
}

// 対称性を利用した動き生成
fn generate_leaper_moves(dx: i8, dy: i8) -> Vec<Vector2> {
    let mut moves = vec![];
    for &sx in &[1, -1] {
        for &sy in &[1, -1] {
            moves.push(Vector2::new(dx * sx, dy * sy));
            if dx != dy {
                moves.push(Vector2::new(dy * sx, dx * sy));
            }
        }
    }
    moves
}

// 例：各駒の定義
fn knight() -> PieceDefinition {
    PieceDefinition {
        name: "Knight".to_string(),
        movements: vec![
            MovementType::Leap {
                offsets: generate_leaper_moves(1, 2),
            }
        ],
        is_royal: false,
    }
}

fn amazon() -> PieceDefinition {
    PieceDefinition {
        name: "Amazon".to_string(),
        movements: vec![
            // Queen の動き
            MovementType::Slide {
                directions: vec![
                    Vector2::new(1, 0), Vector2::new(-1, 0),
                    Vector2::new(0, 1), Vector2::new(0, -1),
                    Vector2::new(1, 1), Vector2::new(1, -1),
                    Vector2::new(-1, 1), Vector2::new(-1, -1),
                ],
                max_distance: None,
                blocked_by_pieces: true,
            },
            // Knight の動き
            MovementType::Leap {
                offsets: generate_leaper_moves(1, 2),
            }
        ],
        is_royal: false,
    }
}

fn camel() -> PieceDefinition {
    PieceDefinition {
        name: "Camel".to_string(),
        movements: vec![
            MovementType::Leap {
                offsets: generate_leaper_moves(1, 3),  // (1, 3) リーパー
            }
        ],
        is_royal: false,
    }
}
```

### 5.3 トレイト設計

```rust
/// 評価器のトレイト
trait Evaluator {
    /// 局面を評価
    fn evaluate(&self, state: &GameState) -> EvaluationResult;
}

/// 評価結果（スコア + 説明）
struct EvaluationResult {
    /// 数値スコア（正が白有利）
    score: i32,
    /// 評価の内訳
    breakdown: EvaluationBreakdown,
    /// 検出された戦術
    tactics: Vec<DetectedTactic>,
}

/// 評価の内訳
struct EvaluationBreakdown {
    material_estimate: i32,    // 駒価値の推定（動的）
    mobility: i32,             // 機動力
    king_safety: i32,          // キング安全度
    coordination: i32,         // 駒の連携
    center_control: i32,       // センター支配
    threats: i32,              // 脅威スコア
}

/// 検出された戦術
struct DetectedTactic {
    tactic_type: TacticType,
    description: String,
    involved_pieces: Vec<Piece>,
    evaluation_impact: i32,
}

/// 動き生成器のトレイト
trait MoveGenerator {
    fn generate_legal_moves(&self, state: &GameState) -> Vec<Move>;
    fn generate_captures(&self, state: &GameState) -> Vec<Move>;
    fn is_in_check(&self, state: &GameState, color: Color) -> bool;
}

/// バリアントのトレイト
trait Variant {
    fn piece_definitions(&self) -> &[PieceDefinition];
    fn initial_position(&self) -> GameState;
    fn is_game_over(&self, state: &GameState) -> Option<GameResult>;
}
```

### 5.4 Bitboard の役割

```rust
/// 64ビット Bitboard
type Bitboard = u64;

/// Bitboard 操作
struct BitboardOps;

impl BitboardOps {
    /// マスをビットに変換
    fn square_to_bit(sq: Square) -> Bitboard {
        1u64 << sq.index()
    }
    
    /// 攻撃マップの生成
    fn generate_attacks(piece: &Piece, blockers: Bitboard) -> Bitboard {
        // レイキャストで攻撃可能マスを計算
        // ...
    }
    
    /// popcount（立っているビット数）
    fn popcount(bb: Bitboard) -> u32 {
        bb.count_ones()
    }
}

// Bitboard を使った高速な脅威検出
fn detect_threats_fast(state: &GameState) -> Vec<Threat> {
    let mut threats = vec![];
    
    for piece in state.pieces() {
        let attacks = BitboardOps::generate_attacks(piece, state.all_pieces());
        let enemy_pieces = attacks & state.pieces_of(piece.color.opposite());
        
        // enemy_pieces の各ビットが脅威
        for target_sq in BitIterator::new(enemy_pieces) {
            threats.push(Threat {
                origin: piece.square,
                vector: target_sq - piece.square,
                reward: ThreatReward::Capture {
                    piece_value: state.piece_at(target_sq).value(),
                },
            });
        }
    }
    
    threats
}
```

---

## 6. Evaluation Function Design

### 6.1 グラフ特徴量の定義

```rust
/// グラフから抽出する特徴量
struct GraphFeatures {
    // 駒ごとの特徴
    piece_mobility: HashMap<PieceId, u32>,      // 到達可能マス数
    piece_centrality: HashMap<PieceId, f32>,    // 中心性スコア
    piece_threats: HashMap<PieceId, u32>,       // 生成する脅威数
    
    // グローバル特徴
    connectivity: f32,          // 防御ネットワークの連結度
    center_control: i32,        // センター支配（味方 - 敵）
    king_zone_attacks: u32,     // 敵キング周辺への攻撃数
}

fn extract_features(state: &GameState) -> GraphFeatures {
    let graph = PositionGraph::from_state(state);
    
    GraphFeatures {
        piece_mobility: graph.compute_mobility_all(),
        piece_centrality: graph.compute_centrality_all(),
        piece_threats: graph.count_threats_all(),
        connectivity: graph.defense_connectivity(),
        center_control: graph.center_control_balance(),
        king_zone_attacks: graph.attacks_on_king_zone(Color::Black),
    }
}
```

### 6.2 駒価値の動的導出

**駒の価値を手動設定せず、グラフ特徴から導出**

```rust
/// 駒の動的価値を計算
fn dynamic_piece_value(piece: &Piece, features: &GraphFeatures) -> i32 {
    let mobility = features.piece_mobility.get(&piece.id).unwrap_or(&0);
    let centrality = features.piece_centrality.get(&piece.id).unwrap_or(&0.0);
    let threats = features.piece_threats.get(&piece.id).unwrap_or(&0);
    
    // 重み（自己対戦で調整可能）
    const W_MOBILITY: i32 = 10;
    const W_CENTRALITY: i32 = 50;
    const W_THREATS: i32 = 20;
    
    (W_MOBILITY * *mobility as i32) +
    (W_CENTRALITY * (*centrality * 100.0) as i32) +
    (W_THREATS * *threats as i32)
}

/// 全体の評価
fn evaluate(state: &GameState) -> EvaluationResult {
    let features = extract_features(state);
    let tactics = detect_tactics(state);
    
    // 駒価値の合計（動的に計算）
    let mut material = 0;
    for piece in state.pieces() {
        let value = dynamic_piece_value(piece, &features);
        material += if piece.color == Color::White { value } else { -value };
    }
    
    // その他の評価項目
    let king_safety = evaluate_king_safety(state, &features);
    let coordination = (features.connectivity * 100.0) as i32;
    
    // 戦術ボーナス
    let tactic_bonus: i32 = tactics.iter()
        .map(|t| t.evaluation_impact)
        .sum();
    
    let total = material + king_safety + coordination + tactic_bonus;
    
    EvaluationResult {
        score: total,
        breakdown: EvaluationBreakdown {
            material_estimate: material,
            mobility: 0,  // TODO
            king_safety,
            coordination,
            center_control: features.center_control,
            threats: tactic_bonus,
        },
        tactics,
    }
}
```

### 6.3 評価の説明生成

```rust
/// 評価を人間が読める形で説明
fn explain_evaluation(result: &EvaluationResult) -> String {
    let mut explanation = String::new();
    
    explanation.push_str(&format!(
        "評価: {} ({})\n",
        result.score,
        if result.score > 0 { "白有利" } else if result.score < 0 { "黒有利" } else { "互角" }
    ));
    
    explanation.push_str("\n内訳:\n");
    explanation.push_str(&format!("  駒価値推定: {}\n", result.breakdown.material_estimate));
    explanation.push_str(&format!("  キング安全度: {}\n", result.breakdown.king_safety));
    explanation.push_str(&format!("  駒の連携: {}\n", result.breakdown.coordination));
    explanation.push_str(&format!("  センター支配: {}\n", result.breakdown.center_control));
    
    if !result.tactics.is_empty() {
        explanation.push_str("\n検出された戦術:\n");
        for tactic in &result.tactics {
            explanation.push_str(&format!("  - {} ({})\n", tactic.description, tactic.tactic_type));
        }
    }
    
    explanation
}
```

---

## 7. Implementation Phases

### Phase 0: プロジェクト基盤（1週間）

**目標**: 開発環境とインフラの整備

- [ ] Cargo プロジェクト初期化
- [ ] GitHub 連携（CI/CD）
- [ ] ディレクトリ構造作成
- [ ] 基本的な型定義（Square, Vector2, Color）
- [ ] テストフレームワーク設定
- [ ] ベンチマークフレームワーク設定（criterion）
- [ ] プロファイリング設定（flamegraph）

### Phase 1: 標準チェス動作（2-3週間）

**目標**: 基本的なチェスエンジンが動作

- [ ] Bitboard 実装
- [ ] 駒の動き定義（標準6種）
- [ ] 合法手生成
- [ ] FEN パーサー
- [ ] 基本的な Alpha-Beta 探索
- [ ] UCI プロトコル（最小限）
- [ ] 単純な評価関数（マテリアルのみ）

**マイルストーン 1**: `position startpos` で合法手が生成される

### Phase 2: 解釈可能評価（3-4週間）

**目標**: グラフ理論ベースの評価関数

- [ ] PositionGraph 実装
- [ ] 脅威検出（Threat 構造体）
- [ ] 戦術分類（フォーク、ピン検出）
- [ ] グラフ特徴量抽出（中心性、連結性）
- [ ] 動的駒価値計算
- [ ] 評価の説明生成
- [ ] 評価内訳の表示

**マイルストーン 2**: `explain` コマンドで評価の理由が表示される

### Phase 3: バリアント対応（2-3週間）

**目標**: 新しい駒への自動適応を検証

- [ ] Variant トレイト実装
- [ ] Amazon 駒の追加
- [ ] Camel 駒の追加
- [ ] バリアント用テストスイート
- [ ] 汎化性能の評価

**マイルストーン 3**: Amazon バリアントで Amazon を「強い駒」として有効活用

**マイルストーン 4**: Camel でフォーク・チェックメイトのパターンを発見

### Phase 4: GUI（2週間）

**目標**: 可視化とデバッグ支援

- [ ] 盤面表示（CLI）
- [ ] 評価のグラフィカル表示
- [ ] 脅威の可視化
- [ ] (オプション) Web UI

---

## 8. Testing & Milestones

### ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_knight_moves() {
        let knight = knight();
        let moves = generate_moves(&knight, Square::E4, Bitboard::empty());
        assert_eq!(moves.len(), 8);
    }
    
    #[test]
    fn test_amazon_moves() {
        let amazon = amazon();
        let moves = generate_moves(&amazon, Square::E4, Bitboard::empty());
        // Queen(27) + Knight(8) = 35 (重複除く)
        assert!(moves.len() > 30);
    }
    
    #[test]
    fn test_fork_detection() {
        // 白ナイトが黒クイーンとルークをフォークする局面
        let state = parse_fen("r3k3/8/8/4N3/8/8/8/4K3 w - - 0 1");
        let tactics = detect_tactics(&state);
        
        assert!(tactics.iter().any(|t| 
            matches!(t.tactic_type, TacticType::DivergentMultiThreat)
        ));
    }
    
    #[test]
    fn test_camel_fork() {
        // Camel (1,3 リーパー) によるフォーク
        let state = setup_camel_fork_position();
        let tactics = detect_tactics(&state);
        
        // Camel という名前を知らなくても、フォークを検出できるはず
        assert!(tactics.iter().any(|t| 
            matches!(t.tactic_type, TacticType::DivergentMultiThreat)
        ));
    }
}
```

### 統合テスト

```rust
// tests/variant_test.rs

#[test]
fn test_amazon_variant_evaluation() {
    let variant = AmazonVariant::new();
    let state = variant.initial_position();
    
    // エンジンに数手考えさせる
    let best_move = search(&state, depth: 5);
    
    // Amazon を活用する手を選んでいるか？
    // （具体的な検証方法は実装時に決定）
}

#[test]
fn test_generalization() {
    // 同じ評価関数が異なるバリアントで動作することを確認
    let evaluator = GraphEvaluator::new();
    
    let standard = StandardChess::new().initial_position();
    let amazon = AmazonVariant::new().initial_position();
    let camel = CamelVariant::new().initial_position();
    
    // すべて評価可能（パニックしない）
    let _ = evaluator.evaluate(&standard);
    let _ = evaluator.evaluate(&amazon);
    let _ = evaluator.evaluate(&camel);
}
```

### マイルストーン詳細

| # | マイルストーン | 検証方法 | 合格基準 |
|---|--------------|---------|---------|
| 1 | 標準チェス動作 | perft テスト | perft(5) が正しい数値 |
| 2 | 評価説明 | explain コマンド | 各項目の内訳が表示される |
| 3 | Amazon 活用 | 自己対戦観察 | Amazon を積極的に使う手を選ぶ |
| 4 | Camel 戦術 | 戦術検出テスト | Camel でのフォーク/メイトを検出 |

---

## 9. Profiling & Debug Infrastructure

### プロファイリングツール

```bash
# flamegraph でプロファイル
cargo flamegraph --bench search_bench

# criterion でベンチマーク
cargo bench

# 特定の関数の時間測定
cargo bench -- evaluate
```

### Cargo.toml 設定

```toml
[dev-dependencies]
criterion = "0.5"

[profile.bench]
debug = true  # flamegraph 用

[[bench]]
name = "search_bench"
harness = false
```

### デバッグ機能

```rust
/// 局面のダンプ出力
fn dump_position(state: &GameState) {
    println!("{}", state.to_ascii());
    println!("FEN: {}", state.to_fen());
    println!("Turn: {:?}", state.side_to_move);
}

/// 評価の詳細出力
fn dump_evaluation(state: &GameState, evaluator: &impl Evaluator) {
    let result = evaluator.evaluate(state);
    println!("{}", explain_evaluation(&result));
}

/// 探索のトレースログ
#[cfg(feature = "trace")]
fn trace_search(depth: u32, move_: Move, score: i32) {
    println!("Depth {}: {} = {}", depth, move_, score);
}
```

### CI/CD

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo bench --no-run  # ベンチマークのコンパイル確認
```

---

## 10. Glossary

| 用語 | 定義 |
|------|------|
| **Threat（脅威）** | 攻撃者の位置・方向・報酬で定義される攻撃関係 |
| **Fork（フォーク）** | 一手で複数の脅威を生成する戦術（非直線） |
| **Pin（ピン）** | 一手で複数の脅威を生成する戦術（直線上） |
| **Centrality（中心性）** | グラフ理論における重要度指標 |
| **Connectivity（連結性）** | グラフがどれだけ繋がっているか |
| **Bitboard** | 64ビット整数で盤面状態を表現する手法 |
| **Leaper（リーパー）** | ジャンプで移動する駒（ナイト、キャメル等） |
| **Amazon** | クイーン + ナイトの動きを持つ複合駒 |
| **Camel** | (1, 3) のオフセットで移動するリーパー |

---

## 11. Appendix

### 議論の経緯

このプロジェクトは以下の問いから始まった：

1. **解釈可能性**: なぜ NNUE はブラックボックスなのか？人間が理解できる評価は可能か？
2. **汎化**: 新しい駒を追加したとき、手動調整なしで適切に評価できるか？
3. **シンプルさ**: 最小限の原則で強いエンジンは作れるか？

グラフ理論的アプローチを採用した理由：
- 駒の「関係性」を直接表現できる
- 中心性・連結性などの概念が戦略と対応
- 駒名に依存しない評価が可能

### 参考リソース

- [Stockfish](https://github.com/official-stockfish/Stockfish) - 参考実装
- [chess-programming wiki](https://www.chessprogramming.org/) - チェスプログラミングの知識ベース
- Graph Theory and Chess - 学術的アプローチ

---

*Last updated: 2026-01-17*
