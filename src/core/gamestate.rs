use super::{Board, Color, Coord, Move, MoveFlags, Piece, PieceType, StandardBoard};
use std::fmt;

/// Castling rights for a player.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastlingRights {
    /// Can castle kingside (O-O)
    pub kingside: bool,
    /// Can castle queenside (O-O-O)
    pub queenside: bool,
}

impl CastlingRights {
    /// No castling rights.
    pub const NONE: Self = Self {
        kingside: false,
        queenside: false,
    };

    /// Both castling rights.
    pub const BOTH: Self = Self {
        kingside: true,
        queenside: true,
    };

    /// Has any castling rights.
    pub fn any(&self) -> bool {
        self.kingside || self.queenside
    }
}

/// Complete game state including board position and metadata.
#[derive(Clone, Debug)]
pub struct GameState {
    /// The board with piece positions
    board: Board,
    /// Whose turn it is
    side_to_move: Color,
    /// Castling rights for white
    white_castling: CastlingRights,
    /// Castling rights for black
    black_castling: CastlingRights,
    /// En passant target square (if a pawn just moved two squares)
    en_passant: Option<Coord>,
    /// Halfmove clock for 50-move rule
    halfmove_clock: u32,
    /// Fullmove number (starts at 1, incremented after Black's move)
    fullmove_number: u32,
}

impl GameState {
    /// Creates an empty game state.
    pub fn empty() -> Self {
        Self {
            board: Board::empty(),
            side_to_move: Color::White,
            white_castling: CastlingRights::NONE,
            black_castling: CastlingRights::NONE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    /// Creates the starting position for standard chess.
    pub fn starting_position() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            .expect("Starting position FEN should be valid")
    }

    /// Parses a FEN string into a GameState.
    ///
    /// FEN format: position side castling en_passant halfmove fullmove
    /// Example: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("FEN must have at least 4 parts".to_string());
        }

        // Parse board position
        let mut board = Board::empty();
        let ranks: Vec<&str> = parts[0].split('/').collect();
        if ranks.len() != 8 {
            return Err("FEN board must have 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - rank_idx; // FEN starts from rank 8
            let mut file = 0;

            for ch in rank_str.chars() {
                if ch.is_ascii_digit() {
                    // Empty squares
                    let empty_count = ch.to_digit(10).unwrap() as u8;
                    file += empty_count;
                } else {
                    // Piece
                    if file >= 8 {
                        return Err(format!("Rank {} has too many squares", rank + 1));
                    }
                    let piece = Piece::from_char(ch)
                        .ok_or_else(|| format!("Invalid piece character: {}", ch))?;
                    let coord = Coord::new(file, rank as u8);
                    board.set_piece(&coord, piece);
                    file += 1;
                }
            }

            if file != 8 {
                return Err(format!("Rank {} has {} squares, expected 8", rank + 1, file));
            }
        }

        // Parse side to move
        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(format!("Invalid side to move: {}", parts[1])),
        };

        // Parse castling rights
        let castling = parts[2];
        let white_castling = CastlingRights {
            kingside: castling.contains('K'),
            queenside: castling.contains('Q'),
        };
        let black_castling = CastlingRights {
            kingside: castling.contains('k'),
            queenside: castling.contains('q'),
        };

        // Parse en passant target
        let en_passant = if parts[3] == "-" {
            None
        } else {
            Some(
                StandardBoard::parse_algebraic(parts[3])
                    .ok_or_else(|| format!("Invalid en passant square: {}", parts[3]))?,
            )
        };

        // Parse halfmove clock (optional, defaults to 0)
        let halfmove_clock = if parts.len() > 4 {
            parts[4]
                .parse()
                .map_err(|_| format!("Invalid halfmove clock: {}", parts[4]))?
        } else {
            0
        };

        // Parse fullmove number (optional, defaults to 1)
        let fullmove_number = if parts.len() > 5 {
            parts[5]
                .parse()
                .map_err(|_| format!("Invalid fullmove number: {}", parts[5]))?
        } else {
            1
        };

        Ok(Self {
            board,
            side_to_move,
            white_castling,
            black_castling,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }

    /// Converts the game state to a FEN string.
    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // Board position
        for rank in (0..8).rev() {
            let mut empty_count = 0;
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                if let Some(piece) = self.board.piece_at(&coord) {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank > 0 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        fen.push(match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        // Castling rights
        fen.push(' ');
        let mut castling = String::new();
        if self.white_castling.kingside {
            castling.push('K');
        }
        if self.white_castling.queenside {
            castling.push('Q');
        }
        if self.black_castling.kingside {
            castling.push('k');
        }
        if self.black_castling.queenside {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }
        fen.push_str(&castling);

        // En passant
        fen.push(' ');
        if let Some(ep) = self.en_passant {
            fen.push_str(&ep.to_algebraic());
        } else {
            fen.push('-');
        }

        // Halfmove clock and fullmove number
        fen.push_str(&format!(" {} {}", self.halfmove_clock, self.fullmove_number));

        fen
    }

    // Getters
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn castling_rights(&self, color: Color) -> CastlingRights {
        match color {
            Color::White => self.white_castling,
            Color::Black => self.black_castling,
        }
    }

    pub fn en_passant(&self) -> Option<Coord> {
        self.en_passant
    }

    pub fn halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }

    pub fn fullmove_number(&self) -> u32 {
        self.fullmove_number
    }

    /// Makes a move on the board (without legality checking).
    ///
    /// This is a basic implementation that will be expanded later.
    pub fn make_move(&mut self, mv: &Move) {
        // Handle special moves
        if mv.is_castling() {
            self.make_castling(mv);
        } else if mv.is_en_passant() {
            self.make_en_passant(mv);
        } else {
            // Normal move
            let captured = self.board.move_piece(&mv.from, &mv.to);

            // Handle promotion
            if let MoveFlags::Promotion { piece: promo_type } = mv.flags {
                if let Some(piece) = self.board.piece_at(&mv.to) {
                    let promoted = Piece::new(promo_type, piece.color);
                    self.board.set_piece(&mv.to, promoted);
                }
            }

            // Update halfmove clock
            if captured.is_some()
                || self
                    .board
                    .piece_at(&mv.to)
                    .is_some_and(|p| p.piece_type == PieceType::Pawn)
            {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }
        }

        // Update en passant target
        self.en_passant = None;
        if let Some(piece) = self.board.piece_at(&mv.to) {
            if piece.piece_type == PieceType::Pawn {
                let rank_diff = (mv.to.rank as i8 - mv.from.rank as i8).abs();
                if rank_diff == 2 {
                    // Pawn moved two squares, set en passant target
                    let ep_rank = (mv.from.rank + mv.to.rank) / 2;
                    self.en_passant = Some(Coord::new(mv.from.file, ep_rank));
                }
            }
        }

        // Update castling rights (basic version)
        self.update_castling_rights(mv);

        // Switch side to move
        self.side_to_move = self.side_to_move.opposite();

        // Update fullmove number
        if self.side_to_move == Color::White {
            self.fullmove_number += 1;
        }
    }

    fn make_castling(&mut self, mv: &Move) {
        // Move king
        self.board.move_piece(&mv.from, &mv.to);

        // Move rook
        let (rook_from, rook_to) = if mv.to.file == 6 {
            // Kingside
            (Coord::new(7, mv.from.rank), Coord::new(5, mv.from.rank))
        } else {
            // Queenside
            (Coord::new(0, mv.from.rank), Coord::new(3, mv.from.rank))
        };
        self.board.move_piece(&rook_from, &rook_to);

        self.halfmove_clock += 1;
    }

    fn make_en_passant(&mut self, mv: &Move) {
        // Move the pawn
        self.board.move_piece(&mv.from, &mv.to);

        // Remove the captured pawn
        let captured_rank = mv.from.rank;
        let captured_coord = Coord::new(mv.to.file, captured_rank);
        self.board.remove_piece(&captured_coord);

        self.halfmove_clock = 0;
    }

    fn update_castling_rights(&mut self, mv: &Move) {
        // If king moves, lose all castling rights
        if let Some(piece) = self.board.piece_at(&mv.to) {
            if piece.piece_type == PieceType::King {
                match piece.color {
                    Color::White => self.white_castling = CastlingRights::NONE,
                    Color::Black => self.black_castling = CastlingRights::NONE,
                }
            }
        }

        // If rook moves from starting position, lose that side's castling right
        let update_rook_rights = |coord: &Coord, rights: &mut CastlingRights| {
            if coord.rank == 0 || coord.rank == 7 {
                if coord.file == 0 {
                    rights.queenside = false;
                } else if coord.file == 7 {
                    rights.kingside = false;
                }
            }
        };

        if let Some(piece) = self.board.piece_at(&mv.to) {
            if piece.piece_type == PieceType::Rook {
                match piece.color {
                    Color::White => update_rook_rights(&mv.from, &mut self.white_castling),
                    Color::Black => update_rook_rights(&mv.from, &mut self.black_castling),
                }
            }
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board.to_ascii())?;
        writeln!(f)?;
        writeln!(f, "FEN: {}", self.to_fen())?;
        writeln!(f, "Side to move: {:?}", self.side_to_move)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_position_fen() {
        let game = GameState::starting_position();
        let fen = game.to_fen();
        assert_eq!(
            fen,
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn test_fen_round_trip() {
        let fens = vec![
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
            "8/8/8/8/8/8/8/8 w - - 0 1",
        ];

        for fen in fens {
            let game = GameState::from_fen(fen).unwrap();
            let result = game.to_fen();
            assert_eq!(result, fen, "FEN round trip failed for: {}", fen);
        }
    }

    #[test]
    fn test_fen_parsing() {
        let game =
            GameState::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
                .unwrap();

        assert_eq!(game.side_to_move, Color::Black);
        assert!(game.white_castling.kingside);
        assert!(game.white_castling.queenside);
        assert!(game.black_castling.kingside);
        assert!(game.black_castling.queenside);
        assert_eq!(game.en_passant, Some(Coord::new(4, 2)));
        assert_eq!(game.halfmove_clock, 0);
        assert_eq!(game.fullmove_number, 1);
    }

    #[test]
    fn test_invalid_fen() {
        assert!(GameState::from_fen("invalid").is_err());
        assert!(GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").is_err());
        assert!(GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP w KQkq - 0 1").is_err());
    }

    #[test]
    fn test_castling_rights_none() {
        let game = GameState::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").unwrap();
        assert!(!game.white_castling.any());
        assert!(!game.black_castling.any());
    }

    #[test]
    fn test_board_pieces() {
        let game = GameState::starting_position();

        // Check white pieces
        assert_eq!(
            game.board.piece_at(&Coord::new(0, 0)),
            Some(Piece::new(PieceType::Rook, Color::White))
        );
        assert_eq!(
            game.board.piece_at(&Coord::new(4, 0)),
            Some(Piece::new(PieceType::King, Color::White))
        );

        // Check black pieces
        assert_eq!(
            game.board.piece_at(&Coord::new(0, 7)),
            Some(Piece::new(PieceType::Rook, Color::Black))
        );
        assert_eq!(
            game.board.piece_at(&Coord::new(4, 7)),
            Some(Piece::new(PieceType::King, Color::Black))
        );

        // Check empty squares
        assert_eq!(game.board.piece_at(&Coord::new(4, 3)), None);
    }
}
