# Phase 1: Fancy Magic Bitboard 実装計画

## 概要

8x8 標準チェス用に **Fancy Magic Bitboard** を実装し、スライド駒の攻撃計算を最適化する。
可変ボードサイズ用には**レイキャストによるフォールバック**も提供する。

---

## Fancy Magic Bitboard とは

### Plain Magic vs Fancy Magic

| 方式 | メモリ使用量 | 実装複雑度 | 特徴 |
|------|-------------|-----------|------|
| Plain Magic | ~2.3 MB | 低 | 各マスで独立したテーブル |
| **Fancy Magic** | **~800 KB** | 中 | 共有テーブル + オフセット |

### Fancy Magic の仕組み

```
Plain Magic:
  ATTACKS[64][4096]  ← 各マスで固定サイズ（無駄あり）

Fancy Magic:
  ATTACKS[shared_table]  ← 共有テーブル
  OFFSETS[64]            ← 各マスの開始位置
  
  index = OFFSETS[sq] + magic_index(blockers)
```

**利点:**
- コーナーのマス（攻撃パターンが少ない）はテーブルが小さい
- 中央のマス（攻撃パターンが多い）はテーブルが大きい
- 全体でメモリを効率的に使用

---

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────┐
│                    AttackGenerator trait                │
│  fn rook_attacks(sq, blockers) -> Bitboard              │
│  fn bishop_attacks(sq, blockers) -> Bitboard            │
└─────────────────────┬───────────────────────────────────┘
                      │
        ┌─────────────┴─────────────┐
        │                           │
┌───────▼───────┐           ┌───────▼───────┐
│ FancyMagic    │           │ RayCast       │
│ (8x8 最適化)   │           │ (汎用)        │
│ ~800 KB       │           │ O(n) per ray  │
└───────────────┘           └───────────────┘
```

---

## データ構造

### Bitboard64

```rust
/// 64ビット Bitboard（8x8 ボード用）
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard64(pub u64);

impl Bitboard64 {
    pub const EMPTY: Self = Self(0);
    pub const ALL: Self = Self(!0);
    
    // ビット操作
    pub fn set(&mut self, sq: usize);
    pub fn clear(&mut self, sq: usize);
    pub fn get(&self, sq: usize) -> bool;
    pub fn toggle(&mut self, sq: usize);
    
    // ビット演算
    pub fn popcount(&self) -> u32;
    pub fn lsb(&self) -> Option<usize>;
    pub fn msb(&self) -> Option<usize>;
    pub fn pop_lsb(&mut self) -> Option<usize>;
    
    // イテレーション
    pub fn iter(&self) -> BitboardIter;
    
    // 演算子オーバーロード
    // BitOr, BitAnd, BitXor, Not, Shl, Shr
}
```

### FancyMagic 構造体

```rust
/// Fancy Magic Bitboard の静的データ
pub struct FancyMagic {
    /// ブロッカーマスク
    pub mask: u64,
    /// マジックナンバー
    pub magic: u64,
    /// シフト量 (64 - index_bits)
    pub shift: u8,
    /// 共有テーブル内のオフセット
    pub offset: u32,
}

/// 全マスの Fancy Magic データ
pub static ROOK_MAGICS: [FancyMagic; 64] = [...];
pub static BISHOP_MAGICS: [FancyMagic; 64] = [...];

/// 共有攻撃テーブル
pub static ROOK_ATTACKS: [Bitboard64; ROOK_TABLE_SIZE] = [...];
pub static BISHOP_ATTACKS: [Bitboard64; BISHOP_TABLE_SIZE] = [...];

// テーブルサイズ（Fancy Magic）
const ROOK_TABLE_SIZE: usize = 102_400;   // ~800 KB
const BISHOP_TABLE_SIZE: usize = 5_248;   // ~41 KB
```

### 攻撃ルックアップ

```rust
/// ルーク攻撃を取得（O(1)）
#[inline]
pub fn rook_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    let magic = &ROOK_MAGICS[sq];
    let blockers = occupied.0 & magic.mask;
    let index = ((blockers.wrapping_mul(magic.magic)) >> magic.shift) as usize;
    ROOK_ATTACKS[magic.offset as usize + index]
}

/// ビショップ攻撃を取得（O(1)）
#[inline]
pub fn bishop_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    let magic = &BISHOP_MAGICS[sq];
    let blockers = occupied.0 & magic.mask;
    let index = ((blockers.wrapping_mul(magic.magic)) >> magic.shift) as usize;
    BISHOP_ATTACKS[magic.offset as usize + index]
}

/// クイーン攻撃 = ルーク + ビショップ
#[inline]
pub fn queen_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    rook_attacks(sq, occupied) | bishop_attacks(sq, occupied)
}
```

---

## 実装ステップ

### Step 1: Bitboard64 基本実装
**ファイル:** `src/movegen/bitboard.rs`

- [ ] Bitboard64 構造体
- [ ] ビット操作メソッド（set, clear, get, toggle）
- [ ] popcount, lsb, msb, pop_lsb
- [ ] イテレータ実装
- [ ] 演算子オーバーロード（|, &, ^, !, <<, >>）
- [ ] ユニットテスト

### Step 2: マスク生成
**ファイル:** `src/movegen/masks.rs`

- [ ] ルークのブロッカーマスク生成（端を除く）
- [ ] ビショップのブロッカーマスク生成
- [ ] ファイル/ランクマスク定数
- [ ] 対角線マスク定数
- [ ] ユニットテスト

### Step 3: レイキャスト実装（テーブル生成用）
**ファイル:** `src/movegen/rays.rs`

- [ ] 方向ごとのレイ生成
- [ ] ブロッカーを考慮した攻撃計算
- [ ] すべてのブロッカー配置の列挙
- [ ] ユニットテスト

### Step 4: Fancy Magic 定数
**ファイル:** `src/movegen/magic_constants.rs`

- [ ] 既知のマジックナンバー（Stockfish 等から）
- [ ] シフト量
- [ ] オフセット計算
- [ ] 定数の検証テスト

### Step 5: 攻撃テーブル初期化
**ファイル:** `src/movegen/attacks.rs`

- [ ] 共有テーブルの初期化
- [ ] ルックアップ関数（rook_attacks, bishop_attacks）
- [ ] ナイト攻撃テーブル（単純、64エントリ）
- [ ] キング攻撃テーブル（単純、64エントリ）
- [ ] ポーン攻撃テーブル（色別、128エントリ）
- [ ] ユニットテスト（perft 位置で検証）

### Step 6: レイキャストフォールバック
**ファイル:** `src/movegen/raycast.rs`

- [ ] 汎用レイキャスト（可変ボードサイズ用）
- [ ] BoardGeometry<W, H> 対応
- [ ] ユニットテスト

### Step 7: 統合
**ファイル:** `src/movegen/mod.rs`

- [ ] AttackGenerator トレイト
- [ ] StandardAttacks（8x8 Fancy Magic）
- [ ] GenericAttacks（レイキャスト）
- [ ] モジュールエクスポート

---

## ファイル構成

```
src/movegen/
├── mod.rs              # モジュールエクスポート、トレイト定義
├── bitboard.rs         # Bitboard64 構造体
├── masks.rs            # ブロッカーマスク、ファイル/ランクマスク
├── rays.rs             # レイ生成（テーブル初期化用）
├── magic_constants.rs  # Fancy Magic 定数（マジック、シフト、オフセット）
├── attacks.rs          # 攻撃テーブル、ルックアップ関数
└── raycast.rs          # 汎用レイキャスト（フォールバック）
```

---

## テスト戦略

### ユニットテスト

```rust
#[test]
fn test_rook_attacks_empty_board() {
    let attacks = rook_attacks(E4, Bitboard64::EMPTY);
    assert_eq!(attacks.popcount(), 14); // e-file + 4th rank - e4
}

#[test]
fn test_rook_attacks_with_blockers() {
    let blockers = Bitboard64::from_squares(&[E2, E6, C4, G4]);
    let attacks = rook_attacks(E4, blockers);
    // e2, e3, e5, e6 (縦) + c4, d4, f4, g4 (横) = 8マス
    assert_eq!(attacks.popcount(), 8);
}

#[test]
fn test_bishop_attacks_corner() {
    let attacks = bishop_attacks(A1, Bitboard64::EMPTY);
    assert_eq!(attacks.popcount(), 7); // a1-h8 対角線
}
```

### Perft テスト（統合後）

```rust
#[test]
fn test_perft_startpos() {
    let state = GameState::from_fen(STARTPOS_FEN);
    assert_eq!(perft(&state, 1), 20);
    assert_eq!(perft(&state, 2), 400);
    assert_eq!(perft(&state, 3), 8_902);
    assert_eq!(perft(&state, 4), 197_281);
}
```

---

## 依存関係

```toml
[dependencies]
# 現時点で追加依存なし

[dev-dependencies]
criterion = "0.5"  # 既存
```

---

## マイルストーン

| # | 目標 | 検証方法 | 完了条件 |
|---|------|----------|----------|
| 1 | Bitboard64 動作 | ユニットテスト | set/get/popcount が正しい |
| 2 | マスク生成 | ユニットテスト | ルーク/ビショップのマスクが正しい |
| 3 | レイキャスト | ユニットテスト | 全方向で正しい攻撃を生成 |
| 4 | Magic 定数 | 検証テスト | 衝突がない |
| 5 | 攻撃ルックアップ | ユニットテスト | 全64マスで正しい攻撃 |
| 6 | フォールバック | ユニットテスト | 10x10 でも動作 |

---

## 参考資料

- Chess Programming Wiki: Magic Bitboards
- Stockfish ソースコード（マジック定数）
- Crafty ソースコード（Fancy Magic 実装）

---

## 次のステップ

Phase 1 完了後：
1. **Phase 1.4**: 盤面状態（Board, GameState）
2. **Phase 1.5**: 合法手生成（MoveGenerator）
3. **Phase 1.6**: FEN パーサー
4. **Phase 1.7**: Perft テスト

---

*Created: 2026-01-17*
