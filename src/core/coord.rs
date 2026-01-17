use super::Delta;
use std::fmt;

/// A coordinate on a chess board, independent of board size.
///
/// This represents a position using file (column) and rank (row),
/// without any assumptions about the board dimensions.
/// Validity checking is done by `BoardGeometry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    /// File (column), 0-indexed from left (a=0, b=1, ...)
    pub file: u8,
    /// Rank (row), 0-indexed from bottom (1st rank=0, 2nd rank=1, ...)
    pub rank: u8,
}

impl Coord {
    /// Creates a new coordinate.
    #[inline]
    pub const fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }

    /// Attempts to apply a delta to this coordinate.
    /// Returns None if the result would have negative components.
    ///
    /// Note: This does NOT check board bounds. Use `BoardGeometry::offset()`
    /// for bound-checked movement.
    pub fn try_offset(&self, delta: Delta) -> Option<Self> {
        let new_file = self.file as i16 + delta.dx as i16;
        let new_rank = self.rank as i16 + delta.dy as i16;

        if new_file >= 0
            && new_rank >= 0
            && new_file <= u8::MAX as i16
            && new_rank <= u8::MAX as i16
        {
            Some(Self::new(new_file as u8, new_rank as u8))
        } else {
            None
        }
    }

    /// Returns the delta from this coordinate to another.
    pub fn delta_to(&self, other: Coord) -> Delta {
        Delta::new(
            other.file as i8 - self.file as i8,
            other.rank as i8 - self.rank as i8,
        )
    }

    /// Returns the file as a character ('a', 'b', ..., 'z').
    /// For files >= 26, returns '?' (use `file_string()` for large boards).
    pub fn file_char(&self) -> char {
        if self.file < 26 {
            (b'a' + self.file) as char
        } else {
            '?'
        }
    }

    /// Returns the file as a string (supports files >= 26).
    pub fn file_string(&self) -> String {
        if self.file < 26 {
            self.file_char().to_string()
        } else {
            // For very large boards: aa, ab, ac, ...
            let first = self.file / 26 - 1;
            let second = self.file % 26;
            format!("{}{}", (b'a' + first) as char, (b'a' + second) as char)
        }
    }

    /// Returns the rank as a 1-indexed number string.
    pub fn rank_string(&self) -> String {
        (self.rank + 1).to_string()
    }

    /// Parses a coordinate from algebraic notation (e.g., "e4", "a10", "k12").
    ///
    /// Note: This does NOT validate against board bounds.
    /// The returned coordinate may be invalid for a given board size.
    pub fn from_algebraic(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase();
        if s.is_empty() {
            return None;
        }

        let bytes = s.as_bytes();

        // Parse file: one or two letters
        let (file, rank_start) = if bytes.len() >= 2 && bytes[1].is_ascii_lowercase() {
            // Two-letter file (aa, ab, etc.)
            let first = bytes[0].checked_sub(b'a')?;
            let second = bytes[1].checked_sub(b'a')?;
            let file = (first as u16 + 1) * 26 + second as u16;
            if file > u8::MAX as u16 {
                return None;
            }
            (file as u8, 2)
        } else {
            // Single-letter file
            let file = bytes[0].checked_sub(b'a')?;
            if file >= 26 {
                return None;
            }
            (file, 1)
        };

        // Parse rank: one or more digits
        let rank_str = &s[rank_start..];
        let rank: u8 = rank_str.parse::<u8>().ok()?.checked_sub(1)?;

        Some(Self::new(file, rank))
    }

    /// Returns algebraic notation (e.g., "e4", "a10").
    pub fn to_algebraic(&self) -> String {
        format!("{}{}", self.file_string(), self.rank_string())
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Coord::new(4, 3);
        assert_eq!(c.file, 4);
        assert_eq!(c.rank, 3);
    }

    #[test]
    fn test_try_offset() {
        let c = Coord::new(4, 3);

        // Valid offset
        assert_eq!(c.try_offset(Delta::new(1, 2)), Some(Coord::new(5, 5)));
        assert_eq!(c.try_offset(Delta::new(-2, -1)), Some(Coord::new(2, 2)));

        // Negative result
        assert_eq!(Coord::new(0, 0).try_offset(Delta::new(-1, 0)), None);
        assert_eq!(Coord::new(0, 0).try_offset(Delta::new(0, -1)), None);
    }

    #[test]
    fn test_delta_to() {
        let c1 = Coord::new(2, 3);
        let c2 = Coord::new(5, 1);

        assert_eq!(c1.delta_to(c2), Delta::new(3, -2));
        assert_eq!(c2.delta_to(c1), Delta::new(-3, 2));
    }

    #[test]
    fn test_algebraic_standard() {
        // Standard chess notation
        assert_eq!(Coord::from_algebraic("a1"), Some(Coord::new(0, 0)));
        assert_eq!(Coord::from_algebraic("e4"), Some(Coord::new(4, 3)));
        assert_eq!(Coord::from_algebraic("h8"), Some(Coord::new(7, 7)));

        // Case insensitive
        assert_eq!(Coord::from_algebraic("E4"), Some(Coord::new(4, 3)));

        // To algebraic
        assert_eq!(Coord::new(4, 3).to_algebraic(), "e4");
        assert_eq!(Coord::new(0, 0).to_algebraic(), "a1");
    }

    #[test]
    fn test_algebraic_large_board() {
        // 10x10 board
        assert_eq!(Coord::from_algebraic("a10"), Some(Coord::new(0, 9)));
        assert_eq!(Coord::from_algebraic("j10"), Some(Coord::new(9, 9)));

        // 12x12 board
        assert_eq!(Coord::from_algebraic("l12"), Some(Coord::new(11, 11)));

        // To algebraic with large ranks
        assert_eq!(Coord::new(9, 9).to_algebraic(), "j10");
        assert_eq!(Coord::new(11, 11).to_algebraic(), "l12");
    }

    #[test]
    fn test_algebraic_invalid() {
        assert_eq!(Coord::from_algebraic(""), None);
        assert_eq!(Coord::from_algebraic("a0"), None); // rank 0 doesn't exist
        assert_eq!(Coord::from_algebraic("11"), None); // no file
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Coord::new(4, 3)), "e4");
        assert_eq!(format!("{}", Coord::new(9, 9)), "j10");
    }
}
