use super::{Color, Coord, Piece, PieceType, StandardBoard};
use crate::movegen::Bitboard64;

/// Represents the board state - which piece is on which square.
///
/// This uses a piece-centric representation for simplicity and interpretability.
#[derive(Clone, Debug)]
pub struct Board {
    /// Piece at each square (None if empty)
    squares: [Option<Piece>; 64],
    /// Bitboard for all occupied squares
    occupied: Bitboard64,
    /// Bitboard for white pieces
    white_pieces: Bitboard64,
    /// Bitboard for black pieces
    black_pieces: Bitboard64,
}

impl Board {
    /// Creates an empty board.
    pub fn empty() -> Self {
        Self {
            squares: [None; 64],
            occupied: Bitboard64::EMPTY,
            white_pieces: Bitboard64::EMPTY,
            black_pieces: Bitboard64::EMPTY,
        }
    }

    /// Returns the piece at a given coordinate, if any.
    pub fn piece_at(&self, coord: &Coord) -> Option<Piece> {
        let index = StandardBoard::to_index(coord)?;
        self.squares[index]
    }

    /// Places a piece on the board.
    pub fn set_piece(&mut self, coord: &Coord, piece: Piece) {
        if let Some(index) = StandardBoard::to_index(coord) {
            self.squares[index] = Some(piece);
            self.occupied.set(index);
            match piece.color {
                Color::White => self.white_pieces.set(index),
                Color::Black => self.black_pieces.set(index),
            }
        }
    }

    /// Removes a piece from the board.
    pub fn remove_piece(&mut self, coord: &Coord) -> Option<Piece> {
        if let Some(index) = StandardBoard::to_index(coord) {
            let piece = self.squares[index];
            if let Some(p) = piece {
                self.squares[index] = None;
                self.occupied.clear(index);
                match p.color {
                    Color::White => self.white_pieces.clear(index),
                    Color::Black => self.black_pieces.clear(index),
                }
            }
            piece
        } else {
            None
        }
    }

    /// Moves a piece from one square to another.
    ///
    /// Returns the captured piece, if any.
    pub fn move_piece(&mut self, from: &Coord, to: &Coord) -> Option<Piece> {
        let piece = self.remove_piece(from)?;
        let captured = self.remove_piece(to);
        self.set_piece(to, piece);
        captured
    }

    /// Returns the bitboard of all occupied squares.
    pub fn occupied(&self) -> Bitboard64 {
        self.occupied
    }

    /// Returns the bitboard of pieces for a given color.
    pub fn pieces_of_color(&self, color: Color) -> Bitboard64 {
        match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        }
    }

    /// Finds the king of the given color.
    pub fn find_king(&self, color: Color) -> Option<Coord> {
        for sq in 0..64 {
            if let Some(piece) = self.squares[sq] {
                if piece.color == color && piece.piece_type == PieceType::King {
                    return StandardBoard::from_index(sq);
                }
            }
        }
        None
    }

    /// Returns an iterator over all pieces on the board.
    pub fn pieces(&self) -> impl Iterator<Item = (Coord, Piece)> + '_ {
        self.squares
            .iter()
            .enumerate()
            .filter_map(|(index, piece)| {
                piece.and_then(|p| StandardBoard::from_index(index).map(|coord| (coord, p)))
            })
    }

    /// Returns an ASCII representation of the board.
    pub fn to_ascii(&self) -> String {
        let mut result = String::new();
        for rank in (0..8).rev() {
            result.push_str(&format!("{} ", rank + 1));
            for file in 0..8 {
                let coord = Coord::new(file, rank);
                if let Some(piece) = self.piece_at(&coord) {
                    result.push(piece.to_char());
                } else {
                    result.push('.');
                }
                result.push(' ');
            }
            result.push('\n');
        }
        result.push_str("  a b c d e f g h\n");
        result
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = Board::empty();
        assert_eq!(board.occupied().popcount(), 0);
        assert!(board.find_king(Color::White).is_none());
    }

    #[test]
    fn test_set_and_get_piece() {
        let mut board = Board::empty();
        let e4 = Coord::new(4, 3);
        let piece = Piece::new(PieceType::Knight, Color::White);

        board.set_piece(&e4, piece);
        assert_eq!(board.piece_at(&e4), Some(piece));
        assert_eq!(board.occupied().popcount(), 1);
        assert_eq!(board.white_pieces.popcount(), 1);
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::empty();
        let e4 = Coord::new(4, 3);
        let piece = Piece::new(PieceType::Knight, Color::White);

        board.set_piece(&e4, piece);
        let removed = board.remove_piece(&e4);
        assert_eq!(removed, Some(piece));
        assert_eq!(board.piece_at(&e4), None);
        assert_eq!(board.occupied().popcount(), 0);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::empty();
        let e2 = Coord::new(4, 1);
        let e4 = Coord::new(4, 3);
        let piece = Piece::new(PieceType::Pawn, Color::White);

        board.set_piece(&e2, piece);
        let captured = board.move_piece(&e2, &e4);
        assert_eq!(captured, None);
        assert_eq!(board.piece_at(&e2), None);
        assert_eq!(board.piece_at(&e4), Some(piece));
    }

    #[test]
    fn test_move_piece_with_capture() {
        let mut board = Board::empty();
        let e4 = Coord::new(4, 3);
        let d5 = Coord::new(3, 4);
        let white_pawn = Piece::new(PieceType::Pawn, Color::White);
        let black_pawn = Piece::new(PieceType::Pawn, Color::Black);

        board.set_piece(&e4, white_pawn);
        board.set_piece(&d5, black_pawn);

        let captured = board.move_piece(&e4, &d5);
        assert_eq!(captured, Some(black_pawn));
        assert_eq!(board.piece_at(&d5), Some(white_pawn));
        assert_eq!(board.occupied().popcount(), 1);
    }

    #[test]
    fn test_find_king() {
        let mut board = Board::empty();
        let e1 = Coord::new(4, 0);
        let e8 = Coord::new(4, 7);

        board.set_piece(&e1, Piece::new(PieceType::King, Color::White));
        board.set_piece(&e8, Piece::new(PieceType::King, Color::Black));

        assert_eq!(board.find_king(Color::White), Some(e1));
        assert_eq!(board.find_king(Color::Black), Some(e8));
    }

    #[test]
    fn test_pieces_iterator() {
        let mut board = Board::empty();
        board.set_piece(&Coord::new(0, 0), Piece::new(PieceType::Rook, Color::White));
        board.set_piece(&Coord::new(4, 4), Piece::new(PieceType::Queen, Color::Black));

        let pieces: Vec<_> = board.pieces().collect();
        assert_eq!(pieces.len(), 2);
    }
}
