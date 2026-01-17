use super::Delta;
use std::fmt;

/// Represents a square on the chess board (0-63).
/// Index 0 = a1, Index 63 = h8
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(u8);

impl Square {
    /// Creates a new Square from an index (0-63).
    /// Panics if index >= 64.
    #[inline]
    pub const fn new(index: u8) -> Self {
        assert!(index < 64, "Square index must be < 64");
        Self(index)
    }

    /// Creates a Square from file (0-7) and rank (0-7).
    /// file: a=0, b=1, ..., h=7
    /// rank: 1=0, 2=1, ..., 8=7
    #[inline]
    pub const fn from_file_rank(file: u8, rank: u8) -> Self {
        assert!(file < 8, "File must be < 8");
        assert!(rank < 8, "Rank must be < 8");
        Self(rank * 8 + file)
    }

    /// Returns the square index (0-63).
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Returns the file (0-7) where a=0, h=7.
    #[inline]
    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    /// Returns the rank (0-7) where 1st rank=0, 8th rank=7.
    #[inline]
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }

    /// Returns the file as a character ('a'-'h').
    pub fn file_char(self) -> char {
        (b'a' + self.file()) as char
    }

    /// Returns the rank as a character ('1'-'8').
    pub fn rank_char(self) -> char {
        (b'1' + self.rank()) as char
    }

    /// Parses a square from algebraic notation (e.g., "e4").
    pub fn from_algebraic(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return None;
        }

        let file = bytes[0].checked_sub(b'a')?;
        let rank = bytes[1].checked_sub(b'1')?;

        if file < 8 && rank < 8 {
            Some(Self::from_file_rank(file, rank))
        } else {
            None
        }
    }

    /// Returns algebraic notation (e.g., "e4").
    pub fn to_algebraic(self) -> String {
        format!("{}{}", self.file_char(), self.rank_char())
    }

    /// Attempts to add a Delta to this square.
    /// Returns None if the result is out of bounds.
    pub fn offset(self, delta: Delta) -> Option<Self> {
        let new_file = self.file() as i8 + delta.dx;
        let new_rank = self.rank() as i8 + delta.dy;

        if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
            Some(Self::from_file_rank(new_file as u8, new_rank as u8))
        } else {
            None
        }
    }

    /// Returns the Delta between two squares.
    pub fn delta_to(self, other: Square) -> Delta {
        Delta::new(
            other.file() as i8 - self.file() as i8,
            other.rank() as i8 - self.rank() as i8,
        )
    }

    /// Iterator over all 64 squares.
    pub fn all() -> impl Iterator<Item = Square> {
        (0..64).map(Square::new)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

// Common square constants
impl Square {
    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);

    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);

    pub const E4: Square = Square(28);
    pub const D4: Square = Square(27);
    pub const E5: Square = Square(36);
    pub const D5: Square = Square(35);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_rank() {
        let sq = Square::from_file_rank(4, 3); // e4
        assert_eq!(sq.file(), 4);
        assert_eq!(sq.rank(), 3);
        assert_eq!(sq.index(), 28);
    }

    #[test]
    fn test_algebraic() {
        let sq = Square::from_algebraic("e4").unwrap();
        assert_eq!(sq, Square::E4);
        assert_eq!(sq.to_algebraic(), "e4");

        assert_eq!(Square::from_algebraic("a1").unwrap(), Square::A1);
        assert_eq!(Square::from_algebraic("h8").unwrap(), Square::H8);

        assert!(Square::from_algebraic("i9").is_none());
        assert!(Square::from_algebraic("a0").is_none());
    }

    #[test]
    fn test_offset() {
        let e4 = Square::E4;

        // One square up
        assert_eq!(
            e4.offset(Delta::new(0, 1)),
            Some(Square::from_algebraic("e5").unwrap())
        );

        // Knight move
        assert_eq!(
            e4.offset(Delta::new(1, 2)),
            Some(Square::from_algebraic("f6").unwrap())
        );

        // Out of bounds
        assert_eq!(Square::A1.offset(Delta::new(-1, 0)), None);
        assert_eq!(Square::H8.offset(Delta::new(1, 0)), None);
    }

    #[test]
    fn test_delta_to() {
        let e4 = Square::E4;
        let e6 = Square::from_algebraic("e6").unwrap();

        assert_eq!(e4.delta_to(e6), Delta::new(0, 2));
        assert_eq!(e6.delta_to(e4), Delta::new(0, -2));
    }

    #[test]
    fn test_all() {
        let squares: Vec<_> = Square::all().collect();
        assert_eq!(squares.len(), 64);
        assert_eq!(squares[0], Square::A1);
        assert_eq!(squares[63], Square::H8);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Square::E4), "e4");
        assert_eq!(format!("{}", Square::A1), "a1");
    }
}
