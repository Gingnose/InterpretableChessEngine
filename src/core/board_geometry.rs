use super::{Coord, Delta};
use std::marker::PhantomData;

/// Board geometry with compile-time dimensions.
///
/// This provides board-specific operations like bounds checking,
/// index conversion, and iteration over all squares.
///
/// # Type Parameters
/// - `WIDTH`: Number of files (columns), 1-255
/// - `HEIGHT`: Number of ranks (rows), 1-255
///
/// # Example
/// ```
/// use interpretable_chess_engine::core::{BoardGeometry, Coord};
///
/// type StandardBoard = BoardGeometry<8, 8>;
///
/// let e4 = Coord::new(4, 3);
/// assert!(StandardBoard::is_valid(&e4));
/// assert_eq!(StandardBoard::to_index(&e4), Some(28));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct BoardGeometry<const WIDTH: u8, const HEIGHT: u8> {
    _phantom: PhantomData<()>,
}

impl<const WIDTH: u8, const HEIGHT: u8> BoardGeometry<WIDTH, HEIGHT> {
    /// The width (number of files) of the board.
    pub const WIDTH: u8 = WIDTH;

    /// The height (number of ranks) of the board.
    pub const HEIGHT: u8 = HEIGHT;

    /// Total number of squares on the board.
    pub const SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

    /// Checks if a coordinate is valid (within board bounds).
    #[inline]
    pub fn is_valid(coord: &Coord) -> bool {
        coord.file < WIDTH && coord.rank < HEIGHT
    }

    /// Converts a coordinate to a linear index (rank-major order).
    /// Returns None if the coordinate is out of bounds.
    ///
    /// Index layout (for 8x8):
    /// - a1 = 0, b1 = 1, ..., h1 = 7
    /// - a2 = 8, b2 = 9, ..., h2 = 15
    /// - ...
    /// - a8 = 56, ..., h8 = 63
    #[inline]
    pub fn to_index(coord: &Coord) -> Option<usize> {
        if Self::is_valid(coord) {
            Some((coord.rank as usize) * (WIDTH as usize) + (coord.file as usize))
        } else {
            None
        }
    }

    /// Converts a linear index to a coordinate.
    /// Returns None if the index is out of bounds.
    #[inline]
    pub fn from_index(index: usize) -> Option<Coord> {
        if index < Self::SIZE {
            Some(Coord::new(
                (index % WIDTH as usize) as u8,
                (index / WIDTH as usize) as u8,
            ))
        } else {
            None
        }
    }

    /// Applies a delta to a coordinate, checking board bounds.
    /// Returns None if the result is out of bounds.
    pub fn offset(coord: &Coord, delta: Delta) -> Option<Coord> {
        let new_coord = coord.try_offset(delta)?;
        if Self::is_valid(&new_coord) {
            Some(new_coord)
        } else {
            None
        }
    }

    /// Parses a coordinate from algebraic notation, validating against board bounds.
    pub fn parse_algebraic(s: &str) -> Option<Coord> {
        let coord = Coord::from_algebraic(s)?;
        if Self::is_valid(&coord) {
            Some(coord)
        } else {
            None
        }
    }

    /// Returns an iterator over all valid coordinates on the board.
    /// Iterates in index order (a1, b1, ..., h1, a2, ..., h8 for 8x8).
    pub fn all_coords() -> impl Iterator<Item = Coord> {
        (0..Self::SIZE).map(|i| Self::from_index(i).unwrap())
    }

    /// Returns an iterator over all valid indices on the board.
    pub fn all_indices() -> impl Iterator<Item = usize> {
        0..Self::SIZE
    }

    /// Returns the coordinate at the center of the board (or near-center for even dimensions).
    pub fn center() -> Coord {
        Coord::new(WIDTH / 2, HEIGHT / 2)
    }

    /// Checks if a coordinate is on the edge of the board.
    pub fn is_edge(coord: &Coord) -> bool {
        Self::is_valid(coord)
            && (coord.file == 0
                || coord.file == WIDTH - 1
                || coord.rank == 0
                || coord.rank == HEIGHT - 1)
    }

    /// Checks if a coordinate is in a corner of the board.
    pub fn is_corner(coord: &Coord) -> bool {
        Self::is_valid(coord)
            && (coord.file == 0 || coord.file == WIDTH - 1)
            && (coord.rank == 0 || coord.rank == HEIGHT - 1)
    }

    /// Returns the Manhattan distance from a coordinate to the nearest edge.
    pub fn distance_to_edge(coord: &Coord) -> Option<u8> {
        if !Self::is_valid(coord) {
            return None;
        }
        let to_left = coord.file;
        let to_right = WIDTH - 1 - coord.file;
        let to_bottom = coord.rank;
        let to_top = HEIGHT - 1 - coord.rank;
        Some(to_left.min(to_right).min(to_bottom).min(to_top))
    }
}

/// Type alias for standard 8x8 chess board.
pub type StandardBoard = BoardGeometry<8, 8>;

/// Common coordinates for standard chess.
impl BoardGeometry<8, 8> {
    // First rank (White's back rank)
    pub const A1: Coord = Coord::new(0, 0);
    pub const B1: Coord = Coord::new(1, 0);
    pub const C1: Coord = Coord::new(2, 0);
    pub const D1: Coord = Coord::new(3, 0);
    pub const E1: Coord = Coord::new(4, 0);
    pub const F1: Coord = Coord::new(5, 0);
    pub const G1: Coord = Coord::new(6, 0);
    pub const H1: Coord = Coord::new(7, 0);

    // Eighth rank (Black's back rank)
    pub const A8: Coord = Coord::new(0, 7);
    pub const B8: Coord = Coord::new(1, 7);
    pub const C8: Coord = Coord::new(2, 7);
    pub const D8: Coord = Coord::new(3, 7);
    pub const E8: Coord = Coord::new(4, 7);
    pub const F8: Coord = Coord::new(5, 7);
    pub const G8: Coord = Coord::new(6, 7);
    pub const H8: Coord = Coord::new(7, 7);

    // Center squares
    pub const D4: Coord = Coord::new(3, 3);
    pub const E4: Coord = Coord::new(4, 3);
    pub const D5: Coord = Coord::new(3, 4);
    pub const E5: Coord = Coord::new(4, 4);
}

#[cfg(test)]
mod tests {
    use super::*;

    type Board8x8 = BoardGeometry<8, 8>;
    type Board10x10 = BoardGeometry<10, 10>;
    type Board6x6 = BoardGeometry<6, 6>;

    #[test]
    fn test_constants() {
        assert_eq!(Board8x8::WIDTH, 8);
        assert_eq!(Board8x8::HEIGHT, 8);
        assert_eq!(Board8x8::SIZE, 64);

        assert_eq!(Board10x10::WIDTH, 10);
        assert_eq!(Board10x10::HEIGHT, 10);
        assert_eq!(Board10x10::SIZE, 100);
    }

    #[test]
    fn test_is_valid() {
        // 8x8 board
        assert!(Board8x8::is_valid(&Coord::new(0, 0)));
        assert!(Board8x8::is_valid(&Coord::new(7, 7)));
        assert!(!Board8x8::is_valid(&Coord::new(8, 0)));
        assert!(!Board8x8::is_valid(&Coord::new(0, 8)));

        // 10x10 board
        assert!(Board10x10::is_valid(&Coord::new(9, 9)));
        assert!(!Board10x10::is_valid(&Coord::new(10, 0)));
    }

    #[test]
    fn test_to_index() {
        // 8x8 board
        assert_eq!(Board8x8::to_index(&Coord::new(0, 0)), Some(0)); // a1
        assert_eq!(Board8x8::to_index(&Coord::new(7, 0)), Some(7)); // h1
        assert_eq!(Board8x8::to_index(&Coord::new(0, 1)), Some(8)); // a2
        assert_eq!(Board8x8::to_index(&Coord::new(4, 3)), Some(28)); // e4
        assert_eq!(Board8x8::to_index(&Coord::new(7, 7)), Some(63)); // h8
        assert_eq!(Board8x8::to_index(&Coord::new(8, 0)), None); // out of bounds

        // 10x10 board
        assert_eq!(Board10x10::to_index(&Coord::new(9, 9)), Some(99));
    }

    #[test]
    fn test_from_index() {
        // 8x8 board
        assert_eq!(Board8x8::from_index(0), Some(Coord::new(0, 0)));
        assert_eq!(Board8x8::from_index(7), Some(Coord::new(7, 0)));
        assert_eq!(Board8x8::from_index(8), Some(Coord::new(0, 1)));
        assert_eq!(Board8x8::from_index(28), Some(Coord::new(4, 3)));
        assert_eq!(Board8x8::from_index(63), Some(Coord::new(7, 7)));
        assert_eq!(Board8x8::from_index(64), None);

        // Roundtrip
        for i in 0..64 {
            let coord = Board8x8::from_index(i).unwrap();
            assert_eq!(Board8x8::to_index(&coord), Some(i));
        }
    }

    #[test]
    fn test_offset() {
        let e4 = Coord::new(4, 3);

        // Valid moves
        assert_eq!(
            Board8x8::offset(&e4, Delta::new(0, 1)),
            Some(Coord::new(4, 4))
        ); // e5
        assert_eq!(
            Board8x8::offset(&e4, Delta::new(1, 2)),
            Some(Coord::new(5, 5))
        ); // f6

        // Out of bounds
        assert_eq!(Board8x8::offset(&Coord::new(0, 0), Delta::new(-1, 0)), None);
        assert_eq!(Board8x8::offset(&Coord::new(7, 7), Delta::new(1, 0)), None);

        // Same move, different board size
        let j10 = Coord::new(9, 9);
        assert_eq!(Board8x8::offset(&j10, Delta::new(0, 0)), None); // out of bounds for 8x8
        assert_eq!(
            Board10x10::offset(&j10, Delta::new(0, 0)),
            Some(Coord::new(9, 9))
        ); // valid for 10x10
    }

    #[test]
    fn test_parse_algebraic() {
        assert_eq!(Board8x8::parse_algebraic("e4"), Some(Coord::new(4, 3)));
        assert_eq!(Board8x8::parse_algebraic("h8"), Some(Coord::new(7, 7)));
        assert_eq!(Board8x8::parse_algebraic("i1"), None); // out of bounds for 8x8
        assert_eq!(Board8x8::parse_algebraic("a9"), None); // out of bounds for 8x8

        assert_eq!(Board10x10::parse_algebraic("j10"), Some(Coord::new(9, 9)));
    }

    #[test]
    fn test_all_coords() {
        let coords: Vec<_> = Board8x8::all_coords().collect();
        assert_eq!(coords.len(), 64);
        assert_eq!(coords[0], Coord::new(0, 0));
        assert_eq!(coords[63], Coord::new(7, 7));

        let coords_6x6: Vec<_> = Board6x6::all_coords().collect();
        assert_eq!(coords_6x6.len(), 36);
    }

    #[test]
    fn test_center() {
        assert_eq!(Board8x8::center(), Coord::new(4, 4)); // e5 area
        assert_eq!(Board10x10::center(), Coord::new(5, 5));
        assert_eq!(Board6x6::center(), Coord::new(3, 3));
    }

    #[test]
    fn test_edge_and_corner() {
        // Corners
        assert!(Board8x8::is_corner(&Coord::new(0, 0))); // a1
        assert!(Board8x8::is_corner(&Coord::new(7, 7))); // h8
        assert!(!Board8x8::is_corner(&Coord::new(4, 4))); // e5

        // Edges (but not corners)
        assert!(Board8x8::is_edge(&Coord::new(3, 0))); // d1
        assert!(Board8x8::is_edge(&Coord::new(0, 3))); // a4
        assert!(!Board8x8::is_edge(&Coord::new(4, 4))); // e5
    }

    #[test]
    fn test_distance_to_edge() {
        assert_eq!(Board8x8::distance_to_edge(&Coord::new(0, 0)), Some(0)); // corner
        assert_eq!(Board8x8::distance_to_edge(&Coord::new(3, 0)), Some(0)); // edge
        assert_eq!(Board8x8::distance_to_edge(&Coord::new(1, 1)), Some(1)); // one from edge
        assert_eq!(Board8x8::distance_to_edge(&Coord::new(3, 3)), Some(3)); // d4
        assert_eq!(Board8x8::distance_to_edge(&Coord::new(4, 4)), Some(3)); // e5
    }

    #[test]
    fn test_standard_board_constants() {
        assert_eq!(StandardBoard::A1, Coord::new(0, 0));
        assert_eq!(StandardBoard::E1, Coord::new(4, 0));
        assert_eq!(StandardBoard::H8, Coord::new(7, 7));
        assert_eq!(StandardBoard::E4, Coord::new(4, 3));
    }
}
