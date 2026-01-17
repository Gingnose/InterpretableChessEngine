use super::bitboard::Bitboard64;

/// Generates rook attacks from a given square with given blockers.
///
/// This is a "slow" ray-casting function used for:
/// - Initializing magic bitboard attack tables
/// - Fallback for non-8x8 boards
///
/// For runtime attack generation on 8x8, use the magic bitboard lookup instead.
pub fn rook_attacks_slow(sq: usize, blockers: Bitboard64) -> Bitboard64 {
    let mut attacks = Bitboard64::EMPTY;

    // North
    attacks |= ray_attacks(sq, 8, blockers);
    // South
    attacks |= ray_attacks(sq, -8, blockers);
    // East
    attacks |= ray_attacks_horizontal(sq, 1, blockers);
    // West
    attacks |= ray_attacks_horizontal(sq, -1, blockers);

    attacks
}

/// Generates bishop attacks from a given square with given blockers.
pub fn bishop_attacks_slow(sq: usize, blockers: Bitboard64) -> Bitboard64 {
    let mut attacks = Bitboard64::EMPTY;

    // NE
    attacks |= ray_attacks_diagonal(sq, 9, blockers);
    // NW
    attacks |= ray_attacks_diagonal(sq, 7, blockers);
    // SE
    attacks |= ray_attacks_diagonal(sq, -7, blockers);
    // SW
    attacks |= ray_attacks_diagonal(sq, -9, blockers);

    attacks
}

/// Casts a ray in a given direction (vertical).
fn ray_attacks(sq: usize, delta: i32, blockers: Bitboard64) -> Bitboard64 {
    let mut attacks = Bitboard64::EMPTY;
    let mut current = sq as i32;

    loop {
        current += delta;
        if !(0..64).contains(&current) {
            break;
        }

        let target = current as usize;
        attacks.set(target);

        if blockers.get(target) {
            break;
        }
    }

    attacks
}

/// Casts a ray horizontally (wraps at file boundaries).
fn ray_attacks_horizontal(sq: usize, delta: i32, blockers: Bitboard64) -> Bitboard64 {
    let mut attacks = Bitboard64::EMPTY;
    let file = sq % 8;
    let rank = sq / 8;
    let mut current_file = file as i32;

    loop {
        current_file += delta;
        if !(0..8).contains(&current_file) {
            break;
        }

        let target = rank * 8 + current_file as usize;
        attacks.set(target);

        if blockers.get(target) {
            break;
        }
    }

    attacks
}

/// Casts a ray diagonally (stops at file/rank boundaries).
fn ray_attacks_diagonal(sq: usize, delta: i32, blockers: Bitboard64) -> Bitboard64 {
    let mut attacks = Bitboard64::EMPTY;
    let mut file = (sq % 8) as i32;
    let mut rank = (sq / 8) as i32;

    let file_delta = if delta.abs() == 7 { -1 } else { 1 };
    let rank_delta = if delta > 0 { 1 } else { -1 };

    loop {
        file += file_delta;
        rank += rank_delta;

        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            break;
        }

        let target = (rank * 8 + file) as usize;
        attacks.set(target);

        if blockers.get(target) {
            break;
        }
    }

    attacks
}

/// Generates all blocker configurations for a given mask.
///
/// This is used to create attack tables by iterating over all possible
/// blocker patterns and computing the corresponding attack patterns.
///
/// Returns an iterator over (blocker_bitboard, index) pairs, where index
/// is the subset index (0 to 2^popcount - 1).
pub fn blocker_permutations(mask: Bitboard64) -> impl Iterator<Item = (Bitboard64, usize)> {
    let squares: Vec<usize> = mask.iter().collect();
    let n = squares.len();
    let total = 1 << n;

    (0..total).map(move |index| {
        let mut blockers = Bitboard64::EMPTY;
        for (bit, &sq) in squares.iter().enumerate() {
            if (index & (1 << bit)) != 0 {
                blockers.set(sq);
            }
        }
        (blockers, index)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_attacks_empty() {
        // e4 (28) with no blockers
        let attacks = rook_attacks_slow(28, Bitboard64::EMPTY);

        // Should attack: e-file (8 squares - 1 for e4) + 4th rank (8 squares - 1 for e4)
        // = 7 + 7 = 14
        assert_eq!(attacks.popcount(), 14);
    }

    #[test]
    fn test_rook_attacks_with_blockers() {
        // e4 (28) with blockers at e2 (12), e6 (44), c4 (26), g4 (30)
        let blockers = Bitboard64::from_squares(&[12, 44, 26, 30]);
        let attacks = rook_attacks_slow(28, blockers);

        // Vertical: e3 (20), e5 (36), e6 (44) [blocked after] = 3
        // Horizontal: d4 (27), c4 (26) [blocked after], f4 (29), g4 (30) [blocked after] = 4
        // Total = 7 (but let's verify exact squares)

        // Actually: e2 blocks downward, e6 blocks upward, c4 blocks left, g4 blocks right
        // So attacks: e3,e5,e6 (up including blocker), e2 (down including blocker),
        //             d4,c4 (left including blocker), f4,g4 (right including blocker)
        assert!(attacks.get(20)); // e3
        assert!(attacks.get(36)); // e5
        assert!(attacks.get(44)); // e6
        assert!(attacks.get(12)); // e2
        assert!(attacks.get(27)); // d4
        assert!(attacks.get(26)); // c4
        assert!(attacks.get(29)); // f4
        assert!(attacks.get(30)); // g4
        assert_eq!(attacks.popcount(), 8);
    }

    #[test]
    fn test_bishop_attacks_empty() {
        // d4 (27) with no blockers
        let attacks = bishop_attacks_slow(27, Bitboard64::EMPTY);

        // NE: e5,f6,g7,h8 = 4
        // NW: c5,b6,a7 = 3
        // SE: e3,f2,g1 = 3
        // SW: c3,b2,a1 = 3
        // Total = 13
        assert_eq!(attacks.popcount(), 13);
    }

    #[test]
    fn test_bishop_attacks_with_blockers() {
        // d4 (27) with blockers at f6 (45), b2 (9)
        let blockers = Bitboard64::from_squares(&[45, 9]);
        let attacks = bishop_attacks_slow(27, blockers);

        // NE: e5,f6 (blocked) = 2
        // NW: c5,b6,a7 = 3
        // SE: e3,f2,g1 = 3
        // SW: c3,b2 (blocked) = 2
        // Total = 10
        assert_eq!(attacks.popcount(), 10);
    }

    #[test]
    fn test_blocker_permutations() {
        // Small mask with 3 squares
        let mask = Bitboard64::from_squares(&[0, 1, 2]);
        let perms: Vec<_> = blocker_permutations(mask).collect();

        // Should have 2^3 = 8 permutations
        assert_eq!(perms.len(), 8);

        // First should be empty
        assert_eq!(perms[0].0, Bitboard64::EMPTY);

        // Last should be all three squares
        assert_eq!(perms[7].0.popcount(), 3);
    }

    #[test]
    fn test_blocker_permutations_indices() {
        let mask = Bitboard64::from_squares(&[10, 20]);
        let perms: Vec<_> = blocker_permutations(mask).collect();

        assert_eq!(perms.len(), 4);
        assert_eq!(perms[0].1, 0); // index 0
        assert_eq!(perms[1].1, 1); // index 1
        assert_eq!(perms[2].1, 2); // index 2
        assert_eq!(perms[3].1, 3); // index 3
    }
}
