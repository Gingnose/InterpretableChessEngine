use super::{Color, Delta};

/// Type of chess piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// A chess piece with color and type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    /// Creates a new piece.
    #[inline]
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    /// Returns the character representation (e.g., 'P' for white pawn, 'n' for black knight).
    pub fn to_char(&self) -> char {
        let ch = match self.piece_type {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };
        if self.color == Color::White {
            ch
        } else {
            ch.to_lowercase().next().unwrap()
        }
    }

    /// Parses a piece from character (e.g., 'P', 'n').
    pub fn from_char(ch: char) -> Option<Self> {
        let color = if ch.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        let piece_type = match ch.to_uppercase().next()? {
            'P' => PieceType::Pawn,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'R' => PieceType::Rook,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => return None,
        };

        Some(Self::new(piece_type, color))
    }
}

/// Describes how a piece can move.
///
/// This is designed to be generic across chess variants.
/// Instead of hardcoding piece behavior, we describe movement patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum MovementType {
    /// Sliding movement (Rook, Bishop, Queen).
    ///
    /// The piece moves in straight lines until blocked.
    Slide {
        /// Directions the piece can slide (e.g., [(1,0), (-1,0)] for horizontal).
        directions: Vec<Delta>,
        /// Maximum distance (None = unlimited).
        max_distance: Option<u8>,
    },

    /// Leaping movement (Knight).
    ///
    /// The piece jumps to specific offsets, ignoring blocking pieces.
    Leap {
        /// All possible jump destinations.
        offsets: Vec<Delta>,
    },

    /// Pawn-like movement.
    ///
    /// Special rules for pawns: move forward, capture diagonally.
    Pawn {
        /// Forward direction (depends on color).
        forward: Delta,
        /// Diagonal capture directions.
        captures: Vec<Delta>,
        /// Starting rank for double-move (e.g., rank 1 for white).
        double_move_from_rank: u8,
    },
}

/// Definition of a piece type for a chess variant.
///
/// This allows us to define custom pieces (Amazon, Camel, etc.)
/// without modifying the core engine.
#[derive(Debug, Clone)]
pub struct PieceDefinition {
    /// Display name of the piece.
    pub name: &'static str,
    /// How this piece can move.
    pub movements: Vec<MovementType>,
    /// Whether this is a royal piece (losing it = game over).
    pub is_royal: bool,
}

impl PieceDefinition {
    /// Generates leaper move offsets using symmetry.
    ///
    /// For a knight (1, 2), this generates all 8 positions:
    /// (±1, ±2) and (±2, ±1).
    pub fn generate_leaper_offsets(dx: i8, dy: i8) -> Vec<Delta> {
        let mut offsets = Vec::new();
        for &sx in &[1, -1] {
            for &sy in &[1, -1] {
                offsets.push(Delta::new(dx * sx, dy * sy));
                if dx != dy {
                    offsets.push(Delta::new(dy * sx, dx * sy));
                }
            }
        }
        offsets
    }

    /// Standard chess knight.
    pub fn knight() -> Self {
        Self {
            name: "Knight",
            movements: vec![MovementType::Leap {
                offsets: Self::generate_leaper_offsets(1, 2),
            }],
            is_royal: false,
        }
    }

    /// Standard chess bishop.
    pub fn bishop() -> Self {
        Self {
            name: "Bishop",
            movements: vec![MovementType::Slide {
                directions: vec![
                    Delta::new(1, 1),
                    Delta::new(1, -1),
                    Delta::new(-1, 1),
                    Delta::new(-1, -1),
                ],
                max_distance: None,
            }],
            is_royal: false,
        }
    }

    /// Standard chess rook.
    pub fn rook() -> Self {
        Self {
            name: "Rook",
            movements: vec![MovementType::Slide {
                directions: vec![
                    Delta::new(1, 0),
                    Delta::new(-1, 0),
                    Delta::new(0, 1),
                    Delta::new(0, -1),
                ],
                max_distance: None,
            }],
            is_royal: false,
        }
    }

    /// Standard chess queen.
    pub fn queen() -> Self {
        Self {
            name: "Queen",
            movements: vec![MovementType::Slide {
                directions: vec![
                    // Diagonal
                    Delta::new(1, 1),
                    Delta::new(1, -1),
                    Delta::new(-1, 1),
                    Delta::new(-1, -1),
                    // Orthogonal
                    Delta::new(1, 0),
                    Delta::new(-1, 0),
                    Delta::new(0, 1),
                    Delta::new(0, -1),
                ],
                max_distance: None,
            }],
            is_royal: false,
        }
    }

    /// Standard chess king.
    pub fn king() -> Self {
        Self {
            name: "King",
            movements: vec![MovementType::Slide {
                directions: vec![
                    // Diagonal
                    Delta::new(1, 1),
                    Delta::new(1, -1),
                    Delta::new(-1, 1),
                    Delta::new(-1, -1),
                    // Orthogonal
                    Delta::new(1, 0),
                    Delta::new(-1, 0),
                    Delta::new(0, 1),
                    Delta::new(0, -1),
                ],
                max_distance: Some(1),
            }],
            is_royal: true,
        }
    }

    /// Standard chess pawn (white).
    pub fn pawn_white() -> Self {
        Self {
            name: "Pawn",
            movements: vec![MovementType::Pawn {
                forward: Delta::new(0, 1),
                captures: vec![Delta::new(1, 1), Delta::new(-1, 1)],
                double_move_from_rank: 1, // 2nd rank (0-indexed)
            }],
            is_royal: false,
        }
    }

    /// Standard chess pawn (black).
    pub fn pawn_black() -> Self {
        Self {
            name: "Pawn",
            movements: vec![MovementType::Pawn {
                forward: Delta::new(0, -1),
                captures: vec![Delta::new(1, -1), Delta::new(-1, -1)],
                double_move_from_rank: 6, // 7th rank (0-indexed)
            }],
            is_royal: false,
        }
    }

    /// Amazon piece: Queen + Knight movement.
    pub fn amazon() -> Self {
        Self {
            name: "Amazon",
            movements: vec![
                // Queen movement
                MovementType::Slide {
                    directions: vec![
                        Delta::new(1, 1),
                        Delta::new(1, -1),
                        Delta::new(-1, 1),
                        Delta::new(-1, -1),
                        Delta::new(1, 0),
                        Delta::new(-1, 0),
                        Delta::new(0, 1),
                        Delta::new(0, -1),
                    ],
                    max_distance: None,
                },
                // Knight movement
                MovementType::Leap {
                    offsets: Self::generate_leaper_offsets(1, 2),
                },
            ],
            is_royal: false,
        }
    }

    /// Camel piece: (1, 3) leaper.
    pub fn camel() -> Self {
        Self {
            name: "Camel",
            movements: vec![MovementType::Leap {
                offsets: Self::generate_leaper_offsets(1, 3),
            }],
            is_royal: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_to_char() {
        assert_eq!(Piece::new(PieceType::Pawn, Color::White).to_char(), 'P');
        assert_eq!(Piece::new(PieceType::Knight, Color::Black).to_char(), 'n');
        assert_eq!(Piece::new(PieceType::Queen, Color::White).to_char(), 'Q');
    }

    #[test]
    fn test_piece_from_char() {
        assert_eq!(
            Piece::from_char('P'),
            Some(Piece::new(PieceType::Pawn, Color::White))
        );
        assert_eq!(
            Piece::from_char('n'),
            Some(Piece::new(PieceType::Knight, Color::Black))
        );
        assert_eq!(Piece::from_char('x'), None);
    }

    #[test]
    fn test_roundtrip() {
        let pieces = [
            Piece::new(PieceType::Pawn, Color::White),
            Piece::new(PieceType::Knight, Color::Black),
            Piece::new(PieceType::Queen, Color::White),
        ];

        for piece in &pieces {
            let ch = piece.to_char();
            assert_eq!(Piece::from_char(ch), Some(*piece));
        }
    }

    #[test]
    fn test_leaper_offsets() {
        let offsets = PieceDefinition::generate_leaper_offsets(1, 2);
        assert_eq!(offsets.len(), 8); // Knight has 8 possible moves

        // Check all expected knight moves are present
        assert!(offsets.contains(&Delta::new(1, 2)));
        assert!(offsets.contains(&Delta::new(-1, 2)));
        assert!(offsets.contains(&Delta::new(2, 1)));
        assert!(offsets.contains(&Delta::new(-2, -1)));
    }

    #[test]
    fn test_piece_definitions() {
        let knight = PieceDefinition::knight();
        assert_eq!(knight.name, "Knight");
        assert!(!knight.is_royal);
        assert_eq!(knight.movements.len(), 1);

        let king = PieceDefinition::king();
        assert!(king.is_royal);

        let amazon = PieceDefinition::amazon();
        assert_eq!(amazon.movements.len(), 2); // Queen + Knight
    }

    #[test]
    fn test_camel_offsets() {
        let camel = PieceDefinition::camel();
        if let MovementType::Leap { offsets } = &camel.movements[0] {
            assert_eq!(offsets.len(), 8);
            assert!(offsets.contains(&Delta::new(1, 3)));
            assert!(offsets.contains(&Delta::new(3, 1)));
        } else {
            panic!("Expected Leap movement for Camel");
        }
    }
}
