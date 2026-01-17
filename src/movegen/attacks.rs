use super::bitboard::Bitboard64;
use super::magic_constants::*;
use super::rays::{bishop_attacks_slow, blocker_permutations, rook_attacks_slow};
use std::sync::OnceLock;

/// Global rook attack table (initialized once).
static ROOK_ATTACKS: OnceLock<Vec<Bitboard64>> = OnceLock::new();

/// Global bishop attack table (initialized once).
static BISHOP_ATTACKS: OnceLock<Vec<Bitboard64>> = OnceLock::new();

/// Global knight attack table.
static KNIGHT_ATTACKS: OnceLock<[Bitboard64; 64]> = OnceLock::new();

/// Global king attack table.
static KING_ATTACKS: OnceLock<[Bitboard64; 64]> = OnceLock::new();

/// Global pawn attack tables (indexed by color, then square).
static PAWN_ATTACKS: OnceLock<[[Bitboard64; 64]; 2]> = OnceLock::new();

/// Initializes the rook attack table using Fancy Magic Bitboards.
fn init_rook_attacks() -> Vec<Bitboard64> {
    let mut table = vec![Bitboard64::EMPTY; ROOK_TABLE_SIZE];

    for (sq, fm) in ROOK_FANCY_MAGICS.iter().enumerate() {
        let mask = Bitboard64(fm.mask);

        for (blockers, _) in blocker_permutations(mask) {
            let attacks = rook_attacks_slow(sq, blockers);
            let index = magic_index(blockers.0, fm.magic, fm.shift) as usize;
            let table_index = fm.offset as usize + index;
            table[table_index] = attacks;
        }
    }

    table
}

/// Initializes the bishop attack table using Fancy Magic Bitboards.
fn init_bishop_attacks() -> Vec<Bitboard64> {
    let mut table = vec![Bitboard64::EMPTY; BISHOP_TABLE_SIZE];

    for (sq, fm) in BISHOP_FANCY_MAGICS.iter().enumerate() {
        let mask = Bitboard64(fm.mask);

        for (blockers, _) in blocker_permutations(mask) {
            let attacks = bishop_attacks_slow(sq, blockers);
            let index = magic_index(blockers.0, fm.magic, fm.shift) as usize;
            let table_index = fm.offset as usize + index;
            table[table_index] = attacks;
        }
    }

    table
}

/// Initializes knight attack table (no blockers needed).
fn init_knight_attacks() -> [Bitboard64; 64] {
    let mut table = [Bitboard64::EMPTY; 64];

    // Knight moves: (±1, ±2) and (±2, ±1)
    let deltas = [
        (1, 2),
        (2, 1),
        (-1, 2),
        (-2, 1),
        (1, -2),
        (2, -1),
        (-1, -2),
        (-2, -1),
    ];

    for (sq, attacks) in table.iter_mut().enumerate() {
        let file = sq % 8;
        let rank = sq / 8;

        for (df, dr) in deltas {
            let new_file = file as i32 + df;
            let new_rank = rank as i32 + dr;

            if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                let target = (new_rank * 8 + new_file) as usize;
                attacks.set(target);
            }
        }
    }

    table
}

/// Initializes king attack table (no blockers needed).
fn init_king_attacks() -> [Bitboard64; 64] {
    let mut table = [Bitboard64::EMPTY; 64];

    for (sq, attacks) in table.iter_mut().enumerate() {
        let file = sq % 8;
        let rank = sq / 8;

        // King moves: all 8 directions, distance 1
        for df in -1..=1 {
            for dr in -1..=1 {
                if df == 0 && dr == 0 {
                    continue;
                }

                let new_file = file as i32 + df;
                let new_rank = rank as i32 + dr;

                if (0..8).contains(&new_file) && (0..8).contains(&new_rank) {
                    let target = (new_rank * 8 + new_file) as usize;
                    attacks.set(target);
                }
            }
        }
    }

    table
}

/// Initializes pawn attack tables (color-dependent).
fn init_pawn_attacks() -> [[Bitboard64; 64]; 2] {
    let mut table = [[Bitboard64::EMPTY; 64]; 2];

    for sq in 0..64 {
        let file = sq % 8;
        let rank = sq / 8;

        // White pawns (color index 0): attack diagonally upward
        let mut white_attacks = Bitboard64::EMPTY;
        if rank < 7 {
            if file > 0 {
                white_attacks.set(sq + 7); // Up-left
            }
            if file < 7 {
                white_attacks.set(sq + 9); // Up-right
            }
        }
        table[0][sq] = white_attacks;

        // Black pawns (color index 1): attack diagonally downward
        let mut black_attacks = Bitboard64::EMPTY;
        if rank > 0 {
            if file > 0 {
                black_attacks.set(sq - 9); // Down-left
            }
            if file < 7 {
                black_attacks.set(sq - 7); // Down-right
            }
        }
        table[1][sq] = black_attacks;
    }

    table
}

/// Computes the magic index for a blocker configuration.
#[inline(always)]
fn magic_index(blockers: u64, magic: u64, shift: u8) -> u32 {
    ((blockers.wrapping_mul(magic)) >> shift) as u32
}

/// Returns rook attacks for a given square and blocker configuration.
///
/// This is the main entry point for magic bitboard lookups.
#[inline(always)]
pub fn rook_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    let table = ROOK_ATTACKS.get_or_init(init_rook_attacks);
    let fm = &ROOK_FANCY_MAGICS[sq];
    let blockers = occupied.0 & fm.mask;
    let index = magic_index(blockers, fm.magic, fm.shift) as usize;
    table[fm.offset as usize + index]
}

/// Returns bishop attacks for a given square and blocker configuration.
#[inline(always)]
pub fn bishop_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    let table = BISHOP_ATTACKS.get_or_init(init_bishop_attacks);
    let fm = &BISHOP_FANCY_MAGICS[sq];
    let blockers = occupied.0 & fm.mask;
    let index = magic_index(blockers, fm.magic, fm.shift) as usize;
    table[fm.offset as usize + index]
}

/// Returns queen attacks (rook + bishop).
#[inline(always)]
pub fn queen_attacks(sq: usize, occupied: Bitboard64) -> Bitboard64 {
    rook_attacks(sq, occupied) | bishop_attacks(sq, occupied)
}

/// Returns knight attacks (no blockers).
#[inline(always)]
pub fn knight_attacks(sq: usize) -> Bitboard64 {
    KNIGHT_ATTACKS.get_or_init(init_knight_attacks)[sq]
}

/// Returns king attacks (no blockers).
#[inline(always)]
pub fn king_attacks(sq: usize) -> Bitboard64 {
    KING_ATTACKS.get_or_init(init_king_attacks)[sq]
}

/// Returns pawn attacks for a given color.
///
/// color: 0 = White, 1 = Black
#[inline(always)]
pub fn pawn_attacks(sq: usize, color: usize) -> Bitboard64 {
    PAWN_ATTACKS.get_or_init(init_pawn_attacks)[color][sq]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rook_attacks_empty_board() {
        // e4 (28) with no blockers
        let attacks = rook_attacks(28, Bitboard64::EMPTY);
        assert_eq!(attacks.popcount(), 14); // 7 on file + 7 on rank
    }

    #[test]
    fn test_rook_attacks_with_blockers() {
        // e4 (28) with blockers
        let blockers = Bitboard64::from_squares(&[12, 44, 26, 30]);
        let attacks = rook_attacks(28, blockers);

        // Should attack up to and including blockers
        assert!(attacks.get(12)); // e2
        assert!(attacks.get(44)); // e6
        assert!(attacks.get(26)); // c4
        assert!(attacks.get(30)); // g4
        assert_eq!(attacks.popcount(), 8);
    }

    #[test]
    fn test_bishop_attacks_empty_board() {
        // d4 (27) with no blockers
        let attacks = bishop_attacks(27, Bitboard64::EMPTY);
        assert_eq!(attacks.popcount(), 13); // All diagonals
    }

    #[test]
    fn test_bishop_attacks_with_blockers() {
        // d4 (27) with blockers at f6 (45), b2 (9)
        let blockers = Bitboard64::from_squares(&[45, 9]);
        let attacks = bishop_attacks(27, blockers);

        // Should include blockers but not beyond
        assert!(attacks.get(45)); // f6
        assert!(attacks.get(9)); // b2
        assert!(!attacks.get(53)); // g7 (beyond f6)
        assert!(!attacks.get(1)); // a1 (beyond b2)
    }

    #[test]
    fn test_queen_attacks() {
        // e4 (28) combines rook and bishop
        let attacks = queen_attacks(28, Bitboard64::EMPTY);
        let rook = rook_attacks(28, Bitboard64::EMPTY);
        let bishop = bishop_attacks(28, Bitboard64::EMPTY);
        assert_eq!(attacks, rook | bishop);
    }

    #[test]
    fn test_knight_attacks() {
        // e4 (28) knight
        let attacks = knight_attacks(28);
        assert_eq!(attacks.popcount(), 8); // Knight has 8 possible moves from e4

        // Corner knight (a1)
        let attacks = knight_attacks(0);
        assert_eq!(attacks.popcount(), 2); // c2, b3
    }

    #[test]
    fn test_king_attacks() {
        // e4 (28) king
        let attacks = king_attacks(28);
        assert_eq!(attacks.popcount(), 8); // King has 8 moves from center

        // Corner king (a1)
        let attacks = king_attacks(0);
        assert_eq!(attacks.popcount(), 3); // a2, b1, b2
    }

    #[test]
    fn test_pawn_attacks_white() {
        // e4 (28) white pawn
        let attacks = pawn_attacks(28, 0);
        assert_eq!(attacks.popcount(), 2); // d5, f5
        assert!(attacks.get(35)); // d5
        assert!(attacks.get(37)); // f5
    }

    #[test]
    fn test_pawn_attacks_black() {
        // e4 (28) black pawn
        let attacks = pawn_attacks(28, 1);
        assert_eq!(attacks.popcount(), 2); // d3, f3
        assert!(attacks.get(19)); // d3
        assert!(attacks.get(21)); // f3
    }

    #[test]
    fn test_pawn_attacks_edge() {
        // a4 (24) white pawn
        let attacks = pawn_attacks(24, 0);
        assert_eq!(attacks.popcount(), 1); // Only b5 (no left capture)
        assert!(attacks.get(33)); // b5
    }

    #[test]
    fn test_magic_consistency() {
        // Test that magic lookups give same results as slow raycast
        for sq in [0, 7, 28, 35, 56, 63] {
            let blockers = Bitboard64::from_squares(&[10, 20, 30, 40]);

            let magic_rook = rook_attacks(sq, blockers);
            let slow_rook = rook_attacks_slow(sq, blockers);
            assert_eq!(magic_rook, slow_rook, "Rook mismatch at square {}", sq);

            let magic_bishop = bishop_attacks(sq, blockers);
            let slow_bishop = bishop_attacks_slow(sq, blockers);
            assert_eq!(
                magic_bishop, slow_bishop,
                "Bishop mismatch at square {}",
                sq
            );
        }
    }
}
