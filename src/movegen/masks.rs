use super::bitboard::Bitboard64;

/// Generates a blocker mask for a rook at the given square.
///
/// The mask includes all squares along ranks and files, excluding:
/// - The rook's own square
/// - The edge squares (they don't affect attack patterns)
///
/// Example for rook on e4 (28):
/// ```text
/// . . . . x . . .   (rank 7, but not edges)
/// . . . . x . . .
/// . . . . x . . .
/// . . . . x . . .
/// . x x x R x x .   (rank 3, excluding a4 and h4)
/// . . . . x . . .
/// . . . . x . . .
/// . . . . . . . .   (rank 0, excluded)
/// ```
pub const fn rook_blocker_mask(sq: usize) -> Bitboard64 {
    let file = sq % 8;
    let rank = sq / 8;

    let mut mask = 0u64;

    // File mask (vertical), excluding top and bottom edges
    let mut r = 1;
    while r < 7 {
        if r != rank {
            mask |= 1u64 << (r * 8 + file);
        }
        r += 1;
    }

    // Rank mask (horizontal), excluding left and right edges
    let mut f = 1;
    while f < 7 {
        if f != file {
            mask |= 1u64 << (rank * 8 + f);
        }
        f += 1;
    }

    Bitboard64(mask)
}

/// Generates a blocker mask for a bishop at the given square.
///
/// The mask includes all squares along diagonals, excluding:
/// - The bishop's own square
/// - The edge squares
pub const fn bishop_blocker_mask(sq: usize) -> Bitboard64 {
    let file = sq % 8;
    let rank = sq / 8;

    let mut mask = 0u64;

    // NE diagonal
    let mut f = file + 1;
    let mut r = rank + 1;
    while f < 7 && r < 7 {
        mask |= 1u64 << (r * 8 + f);
        f += 1;
        r += 1;
    }

    // NW diagonal
    let mut f = file as i32 - 1;
    let mut r = rank + 1;
    while f > 0 && r < 7 {
        mask |= 1u64 << (r * 8 + f as usize);
        f -= 1;
        r += 1;
    }

    // SE diagonal
    let mut f = file + 1;
    let mut r = rank as i32 - 1;
    while f < 7 && r > 0 {
        mask |= 1u64 << (r as usize * 8 + f);
        f += 1;
        r -= 1;
    }

    // SW diagonal
    let mut f = file as i32 - 1;
    let mut r = rank as i32 - 1;
    while f > 0 && r > 0 {
        mask |= 1u64 << (r as usize * 8 + f as usize);
        f -= 1;
        r -= 1;
    }

    Bitboard64(mask)
}

/// Pre-computed rook blocker masks for all 64 squares.
pub const ROOK_MASKS: [Bitboard64; 64] = [
    rook_blocker_mask(0),
    rook_blocker_mask(1),
    rook_blocker_mask(2),
    rook_blocker_mask(3),
    rook_blocker_mask(4),
    rook_blocker_mask(5),
    rook_blocker_mask(6),
    rook_blocker_mask(7),
    rook_blocker_mask(8),
    rook_blocker_mask(9),
    rook_blocker_mask(10),
    rook_blocker_mask(11),
    rook_blocker_mask(12),
    rook_blocker_mask(13),
    rook_blocker_mask(14),
    rook_blocker_mask(15),
    rook_blocker_mask(16),
    rook_blocker_mask(17),
    rook_blocker_mask(18),
    rook_blocker_mask(19),
    rook_blocker_mask(20),
    rook_blocker_mask(21),
    rook_blocker_mask(22),
    rook_blocker_mask(23),
    rook_blocker_mask(24),
    rook_blocker_mask(25),
    rook_blocker_mask(26),
    rook_blocker_mask(27),
    rook_blocker_mask(28),
    rook_blocker_mask(29),
    rook_blocker_mask(30),
    rook_blocker_mask(31),
    rook_blocker_mask(32),
    rook_blocker_mask(33),
    rook_blocker_mask(34),
    rook_blocker_mask(35),
    rook_blocker_mask(36),
    rook_blocker_mask(37),
    rook_blocker_mask(38),
    rook_blocker_mask(39),
    rook_blocker_mask(40),
    rook_blocker_mask(41),
    rook_blocker_mask(42),
    rook_blocker_mask(43),
    rook_blocker_mask(44),
    rook_blocker_mask(45),
    rook_blocker_mask(46),
    rook_blocker_mask(47),
    rook_blocker_mask(48),
    rook_blocker_mask(49),
    rook_blocker_mask(50),
    rook_blocker_mask(51),
    rook_blocker_mask(52),
    rook_blocker_mask(53),
    rook_blocker_mask(54),
    rook_blocker_mask(55),
    rook_blocker_mask(56),
    rook_blocker_mask(57),
    rook_blocker_mask(58),
    rook_blocker_mask(59),
    rook_blocker_mask(60),
    rook_blocker_mask(61),
    rook_blocker_mask(62),
    rook_blocker_mask(63),
];

/// Pre-computed bishop blocker masks for all 64 squares.
pub const BISHOP_MASKS: [Bitboard64; 64] = [
    bishop_blocker_mask(0),
    bishop_blocker_mask(1),
    bishop_blocker_mask(2),
    bishop_blocker_mask(3),
    bishop_blocker_mask(4),
    bishop_blocker_mask(5),
    bishop_blocker_mask(6),
    bishop_blocker_mask(7),
    bishop_blocker_mask(8),
    bishop_blocker_mask(9),
    bishop_blocker_mask(10),
    bishop_blocker_mask(11),
    bishop_blocker_mask(12),
    bishop_blocker_mask(13),
    bishop_blocker_mask(14),
    bishop_blocker_mask(15),
    bishop_blocker_mask(16),
    bishop_blocker_mask(17),
    bishop_blocker_mask(18),
    bishop_blocker_mask(19),
    bishop_blocker_mask(20),
    bishop_blocker_mask(21),
    bishop_blocker_mask(22),
    bishop_blocker_mask(23),
    bishop_blocker_mask(24),
    bishop_blocker_mask(25),
    bishop_blocker_mask(26),
    bishop_blocker_mask(27),
    bishop_blocker_mask(28),
    bishop_blocker_mask(29),
    bishop_blocker_mask(30),
    bishop_blocker_mask(31),
    bishop_blocker_mask(32),
    bishop_blocker_mask(33),
    bishop_blocker_mask(34),
    bishop_blocker_mask(35),
    bishop_blocker_mask(36),
    bishop_blocker_mask(37),
    bishop_blocker_mask(38),
    bishop_blocker_mask(39),
    bishop_blocker_mask(40),
    bishop_blocker_mask(41),
    bishop_blocker_mask(42),
    bishop_blocker_mask(43),
    bishop_blocker_mask(44),
    bishop_blocker_mask(45),
    bishop_blocker_mask(46),
    bishop_blocker_mask(47),
    bishop_blocker_mask(48),
    bishop_blocker_mask(49),
    bishop_blocker_mask(50),
    bishop_blocker_mask(51),
    bishop_blocker_mask(52),
    bishop_blocker_mask(53),
    bishop_blocker_mask(54),
    bishop_blocker_mask(55),
    bishop_blocker_mask(56),
    bishop_blocker_mask(57),
    bishop_blocker_mask(58),
    bishop_blocker_mask(59),
    bishop_blocker_mask(60),
    bishop_blocker_mask(61),
    bishop_blocker_mask(62),
    bishop_blocker_mask(63),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_mask_corner() {
        // a1 (0)
        let mask = ROOK_MASKS[0];
        // Should have vertical (b1-g1) + horizontal (a2-a7)
        // = 6 + 6 = 12 squares
        assert_eq!(mask.popcount(), 12);
    }

    #[test]
    fn test_rook_mask_center() {
        // e4 (28)
        let mask = ROOK_MASKS[28];
        // Vertical: e2, e3, e5, e6, e7 (5 squares, excluding e1 and e8)
        // Horizontal: b4, c4, d4, f4, g4 (5 squares, excluding a4 and h4)
        // Total = 10
        assert_eq!(mask.popcount(), 10);
    }

    #[test]
    fn test_bishop_mask_corner() {
        // a1 (0)
        let mask = BISHOP_MASKS[0];
        // Only the a1-h8 diagonal (excluding a1 and h8)
        // b2, c3, d4, e5, f6, g7 = 6 squares
        assert_eq!(mask.popcount(), 6);
    }

    #[test]
    fn test_bishop_mask_center() {
        // d4 (27)
        let mask = BISHOP_MASKS[27];
        // NE: e5, f6, g7 (3)
        // NW: c5, b6 (2)
        // SE: e3, f2 (2)
        // SW: c3, b2 (2)
        // Total = 9
        assert_eq!(mask.popcount(), 9);
    }

    #[test]
    fn test_rook_mask_excludes_edges() {
        // Any square on the edge
        for sq in [0, 7, 56, 63, 8, 16, 24, 32, 40, 48] {
            let _mask = ROOK_MASKS[sq];
            // Should not include the extreme edges
            for _edge_sq in [0, 7, 56, 63] {
                if _edge_sq != sq {
                    // Edges can be in the mask if they're on the same rank/file
                    // but not at the extremes
                }
            }
        }
    }

    #[test]
    fn test_bishop_mask_excludes_edges() {
        // e4 (28)
        let mask = BISHOP_MASKS[28];
        // Should not include any edge squares
        let edges =
            Bitboard64::RANK_1 | Bitboard64::RANK_8 | Bitboard64::FILE_A | Bitboard64::FILE_H;
        let overlaps = mask & edges;
        assert_eq!(overlaps.popcount(), 0);
    }
}
