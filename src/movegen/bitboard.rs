use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

/// A 64-bit bitboard for representing sets of squares on an 8x8 chess board.
///
/// Each bit corresponds to one square:
/// - Bit 0 = a1, Bit 1 = b1, ..., Bit 7 = h1
/// - Bit 8 = a2, ..., Bit 63 = h8
#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Bitboard64(pub u64);

impl Bitboard64 {
    /// Empty bitboard (no squares set).
    pub const EMPTY: Self = Self(0);

    /// Full bitboard (all squares set).
    pub const ALL: Self = Self(!0);

    /// Creates a bitboard from a u64.
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Creates a bitboard from a single square index (0-63).
    #[inline(always)]
    pub const fn from_square(sq: usize) -> Self {
        debug_assert!(sq < 64, "Square index must be < 64");
        Self(1u64 << sq)
    }

    /// Creates a bitboard from multiple square indices.
    pub fn from_squares(squares: &[usize]) -> Self {
        let mut bb = Self::EMPTY;
        for &sq in squares {
            bb.set(sq);
        }
        bb
    }

    /// Sets a bit at the given square index.
    #[inline(always)]
    pub fn set(&mut self, sq: usize) {
        debug_assert!(sq < 64);
        self.0 |= 1u64 << sq;
    }

    /// Clears a bit at the given square index.
    #[inline(always)]
    pub fn clear(&mut self, sq: usize) {
        debug_assert!(sq < 64);
        self.0 &= !(1u64 << sq);
    }

    /// Toggles a bit at the given square index.
    #[inline(always)]
    pub fn toggle(&mut self, sq: usize) {
        debug_assert!(sq < 64);
        self.0 ^= 1u64 << sq;
    }

    /// Returns true if the bit at the given square is set.
    #[inline(always)]
    pub const fn get(&self, sq: usize) -> bool {
        debug_assert!(sq < 64);
        (self.0 & (1u64 << sq)) != 0
    }

    /// Returns true if the bitboard is empty.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the bitboard is non-empty.
    #[inline(always)]
    pub const fn is_not_empty(&self) -> bool {
        self.0 != 0
    }

    /// Returns the number of set bits (population count).
    #[inline(always)]
    pub const fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the index of the least significant bit (LSB), or None if empty.
    #[inline(always)]
    pub const fn lsb(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some(self.0.trailing_zeros() as usize)
        }
    }

    /// Returns the index of the most significant bit (MSB), or None if empty.
    #[inline(always)]
    pub const fn msb(&self) -> Option<usize> {
        if self.0 == 0 {
            None
        } else {
            Some(63 - self.0.leading_zeros() as usize)
        }
    }

    /// Pops the LSB and returns its index, or None if empty.
    ///
    /// This is useful for iterating over all set bits.
    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Option<usize> {
        if let Some(sq) = self.lsb() {
            self.0 &= self.0 - 1; // Clear LSB
            Some(sq)
        } else {
            None
        }
    }

    /// Returns an iterator over all set bit indices.
    pub fn iter(&self) -> BitboardIter {
        BitboardIter { bb: *self }
    }

    /// Shifts the bitboard north (up one rank).
    #[inline(always)]
    pub const fn north(&self) -> Self {
        Self(self.0 << 8)
    }

    /// Shifts the bitboard south (down one rank).
    #[inline(always)]
    pub const fn south(&self) -> Self {
        Self(self.0 >> 8)
    }

    /// Shifts the bitboard east (right one file).
    #[inline(always)]
    pub const fn east(&self) -> Self {
        Self((self.0 << 1) & !Self::FILE_A.0)
    }

    /// Shifts the bitboard west (left one file).
    #[inline(always)]
    pub const fn west(&self) -> Self {
        Self((self.0 >> 1) & !Self::FILE_H.0)
    }

    // File masks
    pub const FILE_A: Self = Self(0x0101010101010101);
    pub const FILE_B: Self = Self(0x0202020202020202);
    pub const FILE_C: Self = Self(0x0404040404040404);
    pub const FILE_D: Self = Self(0x0808080808080808);
    pub const FILE_E: Self = Self(0x1010101010101010);
    pub const FILE_F: Self = Self(0x2020202020202020);
    pub const FILE_G: Self = Self(0x4040404040404040);
    pub const FILE_H: Self = Self(0x8080808080808080);

    // Rank masks
    pub const RANK_1: Self = Self(0x00000000000000FF);
    pub const RANK_2: Self = Self(0x000000000000FF00);
    pub const RANK_3: Self = Self(0x0000000000FF0000);
    pub const RANK_4: Self = Self(0x00000000FF000000);
    pub const RANK_5: Self = Self(0x000000FF00000000);
    pub const RANK_6: Self = Self(0x0000FF0000000000);
    pub const RANK_7: Self = Self(0x00FF000000000000);
    pub const RANK_8: Self = Self(0xFF00000000000000);

    /// Returns the file mask for a given square.
    pub const fn file_mask(sq: usize) -> Self {
        Self(Self::FILE_A.0 << (sq % 8))
    }

    /// Returns the rank mask for a given square.
    pub const fn rank_mask(sq: usize) -> Self {
        Self(Self::RANK_1.0 << (8 * (sq / 8)))
    }
}

/// Iterator over set bits in a bitboard.
pub struct BitboardIter {
    bb: Bitboard64,
}

impl Iterator for BitboardIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.bb.pop_lsb()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.bb.popcount() as usize;
        (count, Some(count))
    }
}

impl ExactSizeIterator for BitboardIter {}

// Bitwise operators
impl BitOr for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard64 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard64 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard64 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Shl<usize> for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<usize> for Bitboard64 {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl fmt::Debug for Bitboard64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bitboard64({:#018x})", self.0)
    }
}

impl fmt::Display for Bitboard64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                if self.get(sq) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_and_all() {
        assert_eq!(Bitboard64::EMPTY.0, 0);
        assert_eq!(Bitboard64::ALL.0, !0);
        assert!(Bitboard64::EMPTY.is_empty());
        assert!(!Bitboard64::ALL.is_empty());
    }

    #[test]
    fn test_from_square() {
        let bb = Bitboard64::from_square(0); // a1
        assert_eq!(bb.0, 1);

        let bb = Bitboard64::from_square(63); // h8
        assert_eq!(bb.0, 1u64 << 63);
    }

    #[test]
    fn test_set_clear_get() {
        let mut bb = Bitboard64::EMPTY;
        assert!(!bb.get(0));

        bb.set(0);
        assert!(bb.get(0));
        assert_eq!(bb.0, 1);

        bb.set(7);
        assert!(bb.get(7));

        bb.clear(0);
        assert!(!bb.get(0));
        assert!(bb.get(7));
    }

    #[test]
    fn test_toggle() {
        let mut bb = Bitboard64::EMPTY;
        bb.toggle(5);
        assert!(bb.get(5));
        bb.toggle(5);
        assert!(!bb.get(5));
    }

    #[test]
    fn test_popcount() {
        assert_eq!(Bitboard64::EMPTY.popcount(), 0);
        assert_eq!(Bitboard64::ALL.popcount(), 64);

        let bb = Bitboard64::from_squares(&[0, 1, 2]);
        assert_eq!(bb.popcount(), 3);
    }

    #[test]
    fn test_lsb_msb() {
        let bb = Bitboard64::from_squares(&[5, 10, 20]);
        assert_eq!(bb.lsb(), Some(5));
        assert_eq!(bb.msb(), Some(20));

        assert_eq!(Bitboard64::EMPTY.lsb(), None);
        assert_eq!(Bitboard64::EMPTY.msb(), None);
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = Bitboard64::from_squares(&[1, 5, 10]);

        assert_eq!(bb.pop_lsb(), Some(1));
        assert_eq!(bb.pop_lsb(), Some(5));
        assert_eq!(bb.pop_lsb(), Some(10));
        assert_eq!(bb.pop_lsb(), None);
    }

    #[test]
    fn test_iterator() {
        let bb = Bitboard64::from_squares(&[0, 5, 10, 20]);
        let squares: Vec<_> = bb.iter().collect();
        assert_eq!(squares, vec![0, 5, 10, 20]);
    }

    #[test]
    fn test_bitwise_ops() {
        let a = Bitboard64::from_squares(&[0, 1, 2]);
        let b = Bitboard64::from_squares(&[1, 2, 3]);

        let or = a | b;
        assert_eq!(or.popcount(), 4); // 0,1,2,3

        let and = a & b;
        assert_eq!(and.popcount(), 2); // 1,2

        let xor = a ^ b;
        assert_eq!(xor.popcount(), 2); // 0,3

        let not_a = !a;
        assert_eq!(not_a.popcount(), 61); // All except 0,1,2
    }

    #[test]
    fn test_shifts() {
        let bb = Bitboard64::from_square(28); // e4

        let north = bb.north();
        assert!(north.get(36)); // e5

        let south = bb.south();
        assert!(south.get(20)); // e3

        let east = bb.east();
        assert!(east.get(29)); // f4

        let west = bb.west();
        assert!(west.get(27)); // d4
    }

    #[test]
    fn test_file_masks() {
        assert_eq!(Bitboard64::FILE_A.popcount(), 8);
        assert_eq!(Bitboard64::FILE_H.popcount(), 8);

        // a-file should contain a1, a2, ..., a8
        for rank in 0..8 {
            assert!(Bitboard64::FILE_A.get(rank * 8));
        }
    }

    #[test]
    fn test_rank_masks() {
        assert_eq!(Bitboard64::RANK_1.popcount(), 8);
        assert_eq!(Bitboard64::RANK_8.popcount(), 8);

        // 1st rank should contain a1, b1, ..., h1
        for file in 0..8 {
            assert!(Bitboard64::RANK_1.get(file));
        }
    }

    #[test]
    fn test_file_rank_mask_functions() {
        let e4 = 28; // e4 = file 4, rank 3

        let file_mask = Bitboard64::file_mask(e4);
        assert_eq!(file_mask, Bitboard64::FILE_E);

        let rank_mask = Bitboard64::rank_mask(e4);
        assert_eq!(rank_mask, Bitboard64::RANK_4);
    }
}
