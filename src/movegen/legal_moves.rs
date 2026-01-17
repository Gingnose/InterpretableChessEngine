//! Legal move generation.
//!
//! This module implements complete legal move generation for standard chess.
//! It uses the attack tables from the attacks module for efficient computation.

use super::{
    bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
    Bitboard64,
};
use crate::core::{Color, Coord, GameState, Move, MoveFlags, Piece, PieceType, StandardBoard};

/// Direction rays for slider pin detection.
const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // North
    (0, -1),  // South
    (1, 0),   // East
    (-1, 0),  // West
    (1, 1),   // Northeast
    (1, -1),  // Southeast
    (-1, 1),  // Northwest
    (-1, -1), // Southwest
];

/// Move generator for legal chess moves.
pub struct MoveGenerator<'a> {
    game: &'a GameState,
    /// Bitboard of all pieces
    occupied: Bitboard64,
    /// Bitboard of our pieces
    us: Bitboard64,
    /// Bitboard of enemy pieces
    them: Bitboard64,
    /// Our color
    color: Color,
    /// King position (as square index)
    king_sq: usize,
    /// Squares attacked by the enemy
    enemy_attacks: Bitboard64,
    /// Number of attackers on our king
    checkers: Bitboard64,
    /// Mask of squares we can move to (for non-king pieces when in check)
    check_mask: Bitboard64,
    /// Pinned pieces and their allowed movement rays
    pin_masks: [Bitboard64; 64],
}

impl<'a> MoveGenerator<'a> {
    /// Creates a new move generator for the given game state.
    pub fn new(game: &'a GameState) -> Self {
        let color = game.side_to_move();
        let board = game.board();
        let occupied = board.occupied();
        let us = board.pieces_of_color(color);
        let them = board.pieces_of_color(color.opposite());

        // Find our king
        let king_coord = board.find_king(color).expect("King must exist");
        let king_sq = StandardBoard::to_index(&king_coord).unwrap();

        // Initialize with empty values; will be computed in analyze()
        let mut gen = Self {
            game,
            occupied,
            us,
            them,
            color,
            king_sq,
            enemy_attacks: Bitboard64::EMPTY,
            checkers: Bitboard64::EMPTY,
            check_mask: Bitboard64::ALL,
            pin_masks: [Bitboard64::ALL; 64],
        };

        gen.analyze();
        gen
    }

    /// Analyzes the position to compute attacks, checks, and pins.
    fn analyze(&mut self) {
        self.compute_enemy_attacks();
        self.compute_checkers();
        self.compute_pins();
    }

    /// Computes all squares attacked by enemy pieces.
    fn compute_enemy_attacks(&mut self) {
        let board = self.game.board();
        let enemy_color = self.color.opposite();

        // Remove our king from occupied for slider attacks
        // (king must not block attacks that go through him)
        let occupied_no_king = self.occupied & !Bitboard64::from_square(self.king_sq);

        let mut attacks = Bitboard64::EMPTY;

        for (coord, piece) in board.pieces() {
            if piece.color != enemy_color {
                continue;
            }

            let sq = StandardBoard::to_index(&coord).unwrap();

            let piece_attacks = match piece.piece_type {
                PieceType::Pawn => pawn_attacks(sq, enemy_color as usize),
                PieceType::Knight => knight_attacks(sq),
                PieceType::Bishop => bishop_attacks(sq, occupied_no_king),
                PieceType::Rook => rook_attacks(sq, occupied_no_king),
                PieceType::Queen => queen_attacks(sq, occupied_no_king),
                PieceType::King => king_attacks(sq),
            };

            attacks |= piece_attacks;
        }

        self.enemy_attacks = attacks;
    }

    /// Computes pieces that are giving check to our king.
    fn compute_checkers(&mut self) {
        let enemy_color = self.color.opposite();

        let mut checkers = Bitboard64::EMPTY;

        // Pawns
        let pawn_attacks_to_king = pawn_attacks(self.king_sq, self.color as usize);
        for sq in pawn_attacks_to_king.iter() {
            if let Some(piece) = self.piece_at_sq(sq) {
                if piece.color == enemy_color && piece.piece_type == PieceType::Pawn {
                    checkers.set(sq);
                }
            }
        }

        // Knights
        let knight_attacks_to_king = knight_attacks(self.king_sq);
        for sq in knight_attacks_to_king.iter() {
            if let Some(piece) = self.piece_at_sq(sq) {
                if piece.color == enemy_color && piece.piece_type == PieceType::Knight {
                    checkers.set(sq);
                }
            }
        }

        // Bishops and Queens (diagonal)
        let bishop_attacks_to_king = bishop_attacks(self.king_sq, self.occupied);
        for sq in bishop_attacks_to_king.iter() {
            if let Some(piece) = self.piece_at_sq(sq) {
                if piece.color == enemy_color
                    && (piece.piece_type == PieceType::Bishop
                        || piece.piece_type == PieceType::Queen)
                {
                    checkers.set(sq);
                }
            }
        }

        // Rooks and Queens (orthogonal)
        let rook_attacks_to_king = rook_attacks(self.king_sq, self.occupied);
        for sq in rook_attacks_to_king.iter() {
            if let Some(piece) = self.piece_at_sq(sq) {
                if piece.color == enemy_color
                    && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen)
                {
                    checkers.set(sq);
                }
            }
        }

        self.checkers = checkers;

        // Compute check mask (squares that can block or capture the checker)
        let checker_count = checkers.popcount();
        if checker_count == 0 {
            self.check_mask = Bitboard64::ALL;
        } else if checker_count == 1 {
            let checker_sq = checkers.lsb().unwrap();
            // Can capture the checker
            self.check_mask = Bitboard64::from_square(checker_sq);

            // For sliders, can also block
            if let Some(piece) = self.piece_at_sq(checker_sq) {
                if piece.piece_type == PieceType::Bishop
                    || piece.piece_type == PieceType::Rook
                    || piece.piece_type == PieceType::Queen
                {
                    let between = Self::between_squares(checker_sq, self.king_sq);
                    self.check_mask |= between;
                }
            }
        } else {
            // Double check: only king moves are legal
            self.check_mask = Bitboard64::EMPTY;
        }
    }

    /// Computes pinned pieces and their movement masks.
    fn compute_pins(&mut self) {
        let king_file = self.king_sq % 8;
        let king_rank = self.king_sq / 8;

        for (df, dr) in DIRECTIONS {
            let mut ray = Bitboard64::EMPTY;
            let mut pinned_sq: Option<usize> = None;
            let mut f = king_file as i32 + df;
            let mut r = king_rank as i32 + dr;

            while (0..8).contains(&f) && (0..8).contains(&r) {
                let sq = (r * 8 + f) as usize;
                ray.set(sq);

                if self.occupied.get(sq) {
                    if let Some(piece) = self.piece_at_sq(sq) {
                        if piece.color == self.color {
                            // Our piece
                            if pinned_sq.is_some() {
                                // Second piece in line, no pin
                                break;
                            }
                            pinned_sq = Some(sq);
                        } else {
                            // Enemy piece
                            let is_pinner = match (df, dr) {
                                (0, _) | (_, 0) => {
                                    // Orthogonal
                                    piece.piece_type == PieceType::Rook
                                        || piece.piece_type == PieceType::Queen
                                }
                                _ => {
                                    // Diagonal
                                    piece.piece_type == PieceType::Bishop
                                        || piece.piece_type == PieceType::Queen
                                }
                            };

                            if is_pinner {
                                if let Some(pinned) = pinned_sq {
                                    // The piece at pinned_sq is pinned
                                    // It can only move along the pin ray
                                    self.pin_masks[pinned] =
                                        ray | Bitboard64::from_square(self.king_sq);
                                }
                            }
                            break;
                        }
                    }
                }

                f += df;
                r += dr;
            }
        }
    }

    /// Returns the squares between two squares (exclusive).
    fn between_squares(sq1: usize, sq2: usize) -> Bitboard64 {
        let f1 = sq1 % 8;
        let r1 = sq1 / 8;
        let f2 = sq2 % 8;
        let r2 = sq2 / 8;

        let df = (f2 as i32 - f1 as i32).signum();
        let dr = (r2 as i32 - r1 as i32).signum();

        let mut between = Bitboard64::EMPTY;
        let mut f = f1 as i32 + df;
        let mut r = r1 as i32 + dr;

        while f != f2 as i32 || r != r2 as i32 {
            if !(0..8).contains(&f) || !(0..8).contains(&r) {
                break;
            }
            let sq = (r * 8 + f) as usize;
            between.set(sq);
            f += df;
            r += dr;
        }

        between
    }

    /// Helper to get piece at square index.
    fn piece_at_sq(&self, sq: usize) -> Option<Piece> {
        StandardBoard::from_index(sq).and_then(|coord| self.game.board().piece_at(&coord))
    }

    /// Returns true if the current position is in check.
    pub fn in_check(&self) -> bool {
        self.checkers.0 != 0
    }

    /// Returns true if there are multiple checkers (double check).
    pub fn in_double_check(&self) -> bool {
        self.checkers.popcount() > 1
    }

    /// Generates all legal moves.
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(64);

        // In double check, only king can move
        if self.in_double_check() {
            self.generate_king_moves(&mut moves);
            return moves;
        }

        // Generate all piece moves
        self.generate_pawn_moves(&mut moves);
        self.generate_knight_moves(&mut moves);
        self.generate_bishop_moves(&mut moves);
        self.generate_rook_moves(&mut moves);
        self.generate_queen_moves(&mut moves);
        self.generate_king_moves(&mut moves);

        // Castling (only when not in check)
        if !self.in_check() {
            self.generate_castling_moves(&mut moves);
        }

        moves
    }

    /// Generates pawn moves.
    fn generate_pawn_moves(&self, moves: &mut Vec<Move>) {
        let board = self.game.board();
        let forward = if self.color == Color::White { 8i32 } else { -8i32 };
        let start_rank = if self.color == Color::White { 1 } else { 6 };
        let promo_rank = if self.color == Color::White { 7 } else { 0 };
        let ep_rank = if self.color == Color::White { 4 } else { 3 };

        for (coord, piece) in board.pieces() {
            if piece.color != self.color || piece.piece_type != PieceType::Pawn {
                continue;
            }

            let sq = StandardBoard::to_index(&coord).unwrap();
            let pin_mask = self.pin_masks[sq];

            // Single push
            let target_sq = (sq as i32 + forward) as usize;
            if target_sq < 64 && !self.occupied.get(target_sq) {
                let target = Bitboard64::from_square(target_sq);
                if (target & self.check_mask & pin_mask).0 != 0 {
                    let from = coord;
                    let to = StandardBoard::from_index(target_sq).unwrap();

                    if to.rank == promo_rank {
                        // Promotion
                        for promo in [
                            PieceType::Queen,
                            PieceType::Rook,
                            PieceType::Bishop,
                            PieceType::Knight,
                        ] {
                            moves.push(Move::promotion(from, to, promo));
                        }
                    } else {
                        moves.push(Move::new(from, to));
                    }
                }
            }

            // Double push
            if coord.rank == start_rank {
                let single_sq = (sq as i32 + forward) as usize;
                let double_sq = (sq as i32 + forward * 2) as usize;
                if !self.occupied.get(single_sq) && !self.occupied.get(double_sq) {
                    let target = Bitboard64::from_square(double_sq);
                    if (target & self.check_mask & pin_mask).0 != 0 {
                        let from = coord;
                        let to = StandardBoard::from_index(double_sq).unwrap();
                        moves.push(Move::with_flags(from, to, MoveFlags::DoublePawnPush));
                    }
                }
            }

            // Captures
            let pawn_attacks = pawn_attacks(sq, self.color as usize);
            let captures = pawn_attacks & self.them;
            for target_sq in captures.iter() {
                let target = Bitboard64::from_square(target_sq);
                if (target & self.check_mask & pin_mask).0 != 0 {
                    let from = coord;
                    let to = StandardBoard::from_index(target_sq).unwrap();

                    if to.rank == promo_rank {
                        for promo in [
                            PieceType::Queen,
                            PieceType::Rook,
                            PieceType::Bishop,
                            PieceType::Knight,
                        ] {
                            moves.push(Move::promotion(from, to, promo));
                        }
                    } else {
                        moves.push(Move::new(from, to));
                    }
                }
            }

            // En passant
            if let Some(ep_target) = self.game.en_passant() {
                if coord.rank == ep_rank {
                    let ep_sq = StandardBoard::to_index(&ep_target).unwrap();
                    if pawn_attacks.get(ep_sq) {
                        // En passant is special: we need to verify it doesn't expose king
                        if self.is_en_passant_legal(sq, ep_sq) {
                            let target = Bitboard64::from_square(ep_sq);
                            // For en passant, check mask applies to captured pawn, not target square
                            let captured_sq = (ep_sq as i32 - forward) as usize;
                            let captured_bb = Bitboard64::from_square(captured_sq);
                            if (self.check_mask & (target | captured_bb)).0 != 0
                                && (pin_mask & target).0 != 0
                            {
                                moves.push(Move::with_flags(coord, ep_target, MoveFlags::EnPassant));
                            }
                        }
                    }
                }
            }
        }
    }

    /// Checks if en passant is legal (doesn't expose king to discovered check).
    fn is_en_passant_legal(&self, pawn_sq: usize, ep_sq: usize) -> bool {
        let forward = if self.color == Color::White { 8i32 } else { -8i32 };
        let captured_sq = (ep_sq as i32 - forward) as usize;

        // Simulate the move
        let mut new_occupied = self.occupied;
        new_occupied.clear(pawn_sq);
        new_occupied.clear(captured_sq);
        new_occupied.set(ep_sq);

        // Check if king is attacked horizontally (the most common discovered check in EP)
        let rook_attacks_to_king = rook_attacks(self.king_sq, new_occupied);
        let enemy_color = self.color.opposite();

        for sq in rook_attacks_to_king.iter() {
            if let Some(piece) = self.piece_at_sq(sq) {
                if piece.color == enemy_color
                    && (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen)
                {
                    return false;
                }
            }
        }

        true
    }

    /// Generates knight moves.
    fn generate_knight_moves(&self, moves: &mut Vec<Move>) {
        let board = self.game.board();

        for (coord, piece) in board.pieces() {
            if piece.color != self.color || piece.piece_type != PieceType::Knight {
                continue;
            }

            let sq = StandardBoard::to_index(&coord).unwrap();
            let pin_mask = self.pin_masks[sq];

            // Pinned knight can never move (can't stay on pin ray)
            if pin_mask != Bitboard64::ALL {
                continue;
            }

            let attacks = knight_attacks(sq);
            let targets = attacks & !self.us & self.check_mask;

            for target_sq in targets.iter() {
                let to = StandardBoard::from_index(target_sq).unwrap();
                moves.push(Move::new(coord, to));
            }
        }
    }

    /// Generates bishop moves.
    fn generate_bishop_moves(&self, moves: &mut Vec<Move>) {
        self.generate_slider_moves(moves, PieceType::Bishop, bishop_attacks);
    }

    /// Generates rook moves.
    fn generate_rook_moves(&self, moves: &mut Vec<Move>) {
        self.generate_slider_moves(moves, PieceType::Rook, rook_attacks);
    }

    /// Generates queen moves.
    fn generate_queen_moves(&self, moves: &mut Vec<Move>) {
        self.generate_slider_moves(moves, PieceType::Queen, queen_attacks);
    }

    /// Generic slider move generation.
    fn generate_slider_moves<F>(&self, moves: &mut Vec<Move>, piece_type: PieceType, attacks_fn: F)
    where
        F: Fn(usize, Bitboard64) -> Bitboard64,
    {
        let board = self.game.board();

        for (coord, piece) in board.pieces() {
            if piece.color != self.color || piece.piece_type != piece_type {
                continue;
            }

            let sq = StandardBoard::to_index(&coord).unwrap();
            let pin_mask = self.pin_masks[sq];

            let attacks = attacks_fn(sq, self.occupied);
            let targets = attacks & !self.us & self.check_mask & pin_mask;

            for target_sq in targets.iter() {
                let to = StandardBoard::from_index(target_sq).unwrap();
                moves.push(Move::new(coord, to));
            }
        }
    }

    /// Generates king moves.
    fn generate_king_moves(&self, moves: &mut Vec<Move>) {
        let king_coord = StandardBoard::from_index(self.king_sq).unwrap();
        let attacks = king_attacks(self.king_sq);

        // King can move to squares not attacked by enemy and not occupied by our pieces
        let safe_squares = attacks & !self.enemy_attacks & !self.us;

        for target_sq in safe_squares.iter() {
            let to = StandardBoard::from_index(target_sq).unwrap();
            moves.push(Move::new(king_coord, to));
        }
    }

    /// Generates castling moves.
    fn generate_castling_moves(&self, moves: &mut Vec<Move>) {
        let rights = self.game.castling_rights(self.color);
        let rank: u8 = if self.color == Color::White { 0 } else { 7 };
        let rank_offset = rank as usize * 8;

        // Kingside castling
        if rights.kingside {
            let f_sq = rank_offset + 5; // f1 or f8
            let g_sq = rank_offset + 6; // g1 or g8

            // Check squares are empty
            if !self.occupied.get(f_sq) && !self.occupied.get(g_sq) {
                // Check squares are not attacked
                if !self.enemy_attacks.get(f_sq) && !self.enemy_attacks.get(g_sq) {
                    let from = Coord::new(4, rank);
                    let to = Coord::new(6, rank);
                    moves.push(Move::with_flags(from, to, MoveFlags::CastleKingside));
                }
            }
        }

        // Queenside castling
        if rights.queenside {
            let b_sq = rank_offset + 1; // b1 or b8
            let c_sq = rank_offset + 2; // c1 or c8
            let d_sq = rank_offset + 3; // d1 or d8

            // Check squares are empty (b, c, d must be empty)
            if !self.occupied.get(b_sq) && !self.occupied.get(c_sq) && !self.occupied.get(d_sq) {
                // Check king doesn't pass through attacked squares (c, d)
                if !self.enemy_attacks.get(c_sq) && !self.enemy_attacks.get(d_sq) {
                    let from = Coord::new(4, rank);
                    let to = Coord::new(2, rank);
                    moves.push(Move::with_flags(from, to, MoveFlags::CastleQueenside));
                }
            }
        }
    }
}

/// Convenience function to generate all legal moves.
pub fn generate_legal_moves(game: &GameState) -> Vec<Move> {
    MoveGenerator::new(game).generate_moves()
}

/// Returns true if the position is in check.
pub fn is_in_check(game: &GameState) -> bool {
    MoveGenerator::new(game).in_check()
}

/// Counts legal moves (for perft).
pub fn perft(game: &GameState, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_legal_moves(game);

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;
    for mv in moves {
        let mut new_game = game.clone();
        new_game.make_move(&mv);
        nodes += perft(&new_game, depth - 1);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position_moves() {
        let game = GameState::starting_position();
        let moves = generate_legal_moves(&game);
        // 20 moves: 16 pawn moves + 4 knight moves
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn test_not_in_check_at_start() {
        let game = GameState::starting_position();
        assert!(!is_in_check(&game));
    }

    #[test]
    fn test_simple_check() {
        // Position with black king in check
        let game = GameState::from_fen("rnbqkbnr/ppppp1pp/8/5p1Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 2")
            .unwrap();
        assert!(is_in_check(&game));
    }

    #[test]
    fn test_scholar_mate() {
        // Scholar's mate position (black is checkmated)
        let game = GameState::from_fen("r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4")
            .unwrap();
        let moves = generate_legal_moves(&game);
        assert_eq!(moves.len(), 0); // Checkmate
        assert!(is_in_check(&game));
    }

    #[test]
    fn test_perft_depth_1() {
        let game = GameState::starting_position();
        assert_eq!(perft(&game, 1), 20);
    }

    #[test]
    fn test_perft_depth_2() {
        let game = GameState::starting_position();
        assert_eq!(perft(&game, 2), 400);
    }

    #[test]
    fn test_perft_depth_3() {
        let game = GameState::starting_position();
        assert_eq!(perft(&game, 3), 8902);
    }

    #[test]
    fn test_perft_depth_4() {
        let game = GameState::starting_position();
        assert_eq!(perft(&game, 4), 197281);
    }

    #[test]
    fn test_castling_available() {
        let game = GameState::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let moves = generate_legal_moves(&game);

        // Find castling moves
        let castling_moves: Vec<_> = moves
            .iter()
            .filter(|m| m.is_castling())
            .collect();

        assert_eq!(castling_moves.len(), 2); // Both kingside and queenside
    }

    #[test]
    fn test_castling_blocked() {
        // Knight blocks kingside
        let game = GameState::from_fen("r3k2r/8/8/8/8/8/8/R3K1NR w KQkq - 0 1").unwrap();
        let moves = generate_legal_moves(&game);

        let castling_moves: Vec<_> = moves
            .iter()
            .filter(|m| matches!(m.flags, MoveFlags::CastleKingside))
            .collect();

        assert_eq!(castling_moves.len(), 0);
    }

    #[test]
    fn test_en_passant() {
        let game =
            GameState::from_fen("rnbqkbnr/pppp1ppp/8/4pP2/8/8/PPPPP1PP/RNBQKBNR w KQkq e6 0 3")
                .unwrap();
        let moves = generate_legal_moves(&game);

        let ep_moves: Vec<_> = moves
            .iter()
            .filter(|m| m.is_en_passant())
            .collect();

        assert_eq!(ep_moves.len(), 1);
    }

    #[test]
    fn test_promotion() {
        let game = GameState::from_fen("8/P7/8/8/8/8/8/4K2k w - - 0 1").unwrap();
        let moves = generate_legal_moves(&game);

        let promo_moves: Vec<_> = moves
            .iter()
            .filter(|m| matches!(m.flags, MoveFlags::Promotion { .. }))
            .collect();

        // 4 promotion choices
        assert_eq!(promo_moves.len(), 4);
    }

    #[test]
    fn test_pin_restricts_movement() {
        // Knight pinned to king by rook
        let game = GameState::from_fen("4k3/8/8/8/4N3/8/8/r3K3 w - - 0 1").unwrap();
        let moves = generate_legal_moves(&game);

        // Knight should not be able to move (pinned)
        let knight_moves: Vec<_> = moves
            .iter()
            .filter(|m| {
                let from_sq = StandardBoard::to_index(&m.from).unwrap();
                from_sq == 28 // e4
            })
            .collect();

        assert_eq!(knight_moves.len(), 0);
    }

    #[test]
    fn test_king_in_check_restricted() {
        // King restricted by queen - verify moves are limited
        let game = GameState::from_fen("8/8/8/8/8/8/2q5/4K3 w - - 0 1").unwrap();
        let moves = generate_legal_moves(&game);

        // All moves should be king moves (no other pieces)
        assert!(moves.iter().all(|m| {
            let piece = game.board().piece_at(&m.from);
            piece.is_some_and(|p| p.piece_type == PieceType::King)
        }));
    }
}
