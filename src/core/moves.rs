use super::{Coord, PieceType};
use std::fmt;

/// Represents a chess move.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    /// Source square.
    pub from: Coord,
    /// Destination square.
    pub to: Coord,
    /// Move flags (promotion, castling, etc.).
    pub flags: MoveFlags,
}

/// Flags indicating special move types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveFlags {
    /// Normal move or capture.
    Normal,
    /// Pawn double-push (enables en passant).
    DoublePawnPush,
    /// En passant capture.
    EnPassant,
    /// Castling kingside.
    CastleKingside,
    /// Castling queenside.
    CastleQueenside,
    /// Pawn promotion.
    Promotion { piece: PieceType },
}

impl Move {
    /// Creates a normal move.
    #[inline]
    pub const fn new(from: Coord, to: Coord) -> Self {
        Self {
            from,
            to,
            flags: MoveFlags::Normal,
        }
    }

    /// Creates a move with specific flags.
    #[inline]
    pub const fn with_flags(from: Coord, to: Coord, flags: MoveFlags) -> Self {
        Self { from, to, flags }
    }

    /// Creates a promotion move.
    #[inline]
    pub const fn promotion(from: Coord, to: Coord, piece: PieceType) -> Self {
        Self {
            from,
            to,
            flags: MoveFlags::Promotion { piece },
        }
    }

    /// Returns true if this is a capture move.
    ///
    /// Note: This requires board context to determine, so it's handled
    /// by the GameState. Here we just check for en passant (always capture).
    #[inline]
    pub const fn is_en_passant(&self) -> bool {
        matches!(self.flags, MoveFlags::EnPassant)
    }

    /// Returns true if this is a castling move.
    #[inline]
    pub const fn is_castling(&self) -> bool {
        matches!(
            self.flags,
            MoveFlags::CastleKingside | MoveFlags::CastleQueenside
        )
    }

    /// Returns true if this is a promotion.
    #[inline]
    pub const fn is_promotion(&self) -> bool {
        matches!(self.flags, MoveFlags::Promotion { .. })
    }

    /// Returns the promoted piece type, if any.
    pub const fn promoted_piece(&self) -> Option<PieceType> {
        if let MoveFlags::Promotion { piece } = self.flags {
            Some(piece)
        } else {
            None
        }
    }

    /// Converts the move to long algebraic notation (e.g., "e2e4", "e7e8q").
    pub fn to_uci(&self) -> String {
        let mut s = format!("{}{}", self.from, self.to);
        if let MoveFlags::Promotion { piece } = self.flags {
            let promotion_char = match piece {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => '?',
            };
            s.push(promotion_char);
        }
        s
    }

    /// Parses a move from UCI notation (e.g., "e2e4", "e7e8q").
    ///
    /// Note: This does NOT validate the move against a board position.
    pub fn from_uci(s: &str) -> Option<Self> {
        if s.len() < 4 {
            return None;
        }

        let from = Coord::from_algebraic(&s[0..2])?;
        let to = Coord::from_algebraic(&s[2..4])?;

        let flags = if s.len() == 5 {
            let promo_char = s.chars().nth(4)?;
            let piece = match promo_char {
                'q' | 'Q' => PieceType::Queen,
                'r' | 'R' => PieceType::Rook,
                'b' | 'B' => PieceType::Bishop,
                'n' | 'N' => PieceType::Knight,
                _ => return None,
            };
            MoveFlags::Promotion { piece }
        } else {
            MoveFlags::Normal
        };

        Some(Self::with_flags(from, to, flags))
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_uci())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_move() {
        let m = Move::new(Coord::new(4, 1), Coord::new(4, 3));
        assert_eq!(m.from, Coord::new(4, 1));
        assert_eq!(m.to, Coord::new(4, 3));
        assert!(!m.is_promotion());
        assert!(!m.is_castling());
    }

    #[test]
    fn test_promotion() {
        let m = Move::promotion(Coord::new(4, 6), Coord::new(4, 7), PieceType::Queen);
        assert!(m.is_promotion());
        assert_eq!(m.promoted_piece(), Some(PieceType::Queen));
    }

    #[test]
    fn test_to_uci() {
        let m = Move::new(Coord::new(4, 1), Coord::new(4, 3));
        assert_eq!(m.to_uci(), "e2e4");

        let promo = Move::promotion(Coord::new(4, 6), Coord::new(4, 7), PieceType::Queen);
        assert_eq!(promo.to_uci(), "e7e8q");
    }

    #[test]
    fn test_from_uci() {
        let m = Move::from_uci("e2e4").unwrap();
        assert_eq!(m.from, Coord::new(4, 1));
        assert_eq!(m.to, Coord::new(4, 3));
        assert!(!m.is_promotion());

        let promo = Move::from_uci("e7e8q").unwrap();
        assert!(promo.is_promotion());
        assert_eq!(promo.promoted_piece(), Some(PieceType::Queen));

        // Invalid
        assert!(Move::from_uci("e2").is_none());
        assert!(Move::from_uci("e2e4x").is_none());
    }

    #[test]
    fn test_roundtrip_uci() {
        let moves = vec![
            Move::new(Coord::new(4, 1), Coord::new(4, 3)),
            Move::promotion(Coord::new(0, 6), Coord::new(0, 7), PieceType::Queen),
            Move::promotion(Coord::new(7, 6), Coord::new(7, 7), PieceType::Knight),
        ];

        for m in moves {
            let uci = m.to_uci();
            let parsed = Move::from_uci(&uci).unwrap();
            assert_eq!(parsed.from, m.from);
            assert_eq!(parsed.to, m.to);
            if let MoveFlags::Promotion { piece } = m.flags {
                assert_eq!(parsed.promoted_piece(), Some(piece));
            }
        }
    }

    #[test]
    fn test_display() {
        let m = Move::new(Coord::new(6, 0), Coord::new(5, 2));
        assert_eq!(format!("{}", m), "g1f3");
    }
}
