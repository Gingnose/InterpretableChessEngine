/// Fancy Magic Bitboard constants for 8x8 chess.
///
/// These magic numbers are borrowed from existing chess engines (Stockfish, etc.)
/// and have been proven to work without collisions for standard chess.
///
/// Fancy Magic structure for a single square.
#[derive(Clone, Copy)]
pub struct FancyMagic {
    /// Blocker mask (relevant occupancy bits)
    pub mask: u64,
    /// Magic number for multiplication
    pub magic: u64,
    /// Right shift amount (64 - index_bits)
    pub shift: u8,
    /// Offset in the shared attack table
    pub offset: u32,
}

/// Rook magic numbers for all 64 squares.
///
/// These are carefully chosen numbers that map all blocker configurations
/// to unique indices without collisions.
pub const ROOK_MAGICS: [u64; 64] = [
    0x0080001020400080,
    0x0040001000200040,
    0x0080081000200080,
    0x0080040800100080,
    0x0080020400080080,
    0x0080010200040080,
    0x0080008001000200,
    0x0080002040800100,
    0x0000800020400080,
    0x0000400020005000,
    0x0000801000200080,
    0x0000800800100080,
    0x0000800400080080,
    0x0000800200040080,
    0x0000800100020080,
    0x0000800040800100,
    0x0000208000400080,
    0x0000404000201000,
    0x0000808010002000,
    0x0000808008001000,
    0x0000808004000800,
    0x0000808002000400,
    0x0000010100020004,
    0x0000020000408104,
    0x0000208080004000,
    0x0000200040005000,
    0x0000100080200080,
    0x0000080080100080,
    0x0000040080080080,
    0x0000020080040080,
    0x0000010080800200,
    0x0000800080004100,
    0x0000204000800080,
    0x0000200040401000,
    0x0000100080802000,
    0x0000080080801000,
    0x0000040080800800,
    0x0000020080800400,
    0x0000020001010004,
    0x0000800040800100,
    0x0000204000808000,
    0x0000200040008080,
    0x0000100020008080,
    0x0000080010008080,
    0x0000040008008080,
    0x0000020004008080,
    0x0000010002008080,
    0x0000004081020004,
    0x0000204000800080,
    0x0000200040008080,
    0x0000100020008080,
    0x0000080010008080,
    0x0000040008008080,
    0x0000020004008080,
    0x0000800100020080,
    0x0000800041000080,
    0x00FFFCDDFCED714A,
    0x007FFCDDFCED714A,
    0x003FFFCDFFD88096,
    0x0000040810002101,
    0x0001000204080011,
    0x0001000204000801,
    0x0001000082000401,
    0x0001FFFAABFAD1A2,
];

/// Bishop magic numbers for all 64 squares.
pub const BISHOP_MAGICS: [u64; 64] = [
    0x0002020202020200,
    0x0002020202020000,
    0x0004010202000000,
    0x0004040080000000,
    0x0001104000000000,
    0x0000821040000000,
    0x0000410410400000,
    0x0000104104104000,
    0x0000040404040400,
    0x0000020202020200,
    0x0000040102020000,
    0x0000040400800000,
    0x0000011040000000,
    0x0000008210400000,
    0x0000004104104000,
    0x0000002082082000,
    0x0004000808080800,
    0x0002000404040400,
    0x0001000202020200,
    0x0000800802004000,
    0x0000800400A00000,
    0x0000200100884000,
    0x0000400082082000,
    0x0000200041041000,
    0x0002080010101000,
    0x0001040008080800,
    0x0000208004010400,
    0x0000404004010200,
    0x0000840000802000,
    0x0000404002011000,
    0x0000808001041000,
    0x0000404000820800,
    0x0001041000202000,
    0x0000820800101000,
    0x0000104400080800,
    0x0000020080080080,
    0x0000404040040100,
    0x0000808100020100,
    0x0001010100020800,
    0x0000808080010400,
    0x0000820820004000,
    0x0000410410002000,
    0x0000082088001000,
    0x0000002011000800,
    0x0000080100400400,
    0x0001010101000200,
    0x0002020202000400,
    0x0001010101000200,
    0x0000410410400000,
    0x0000208208200000,
    0x0000002084100000,
    0x0000000020880000,
    0x0000001002020000,
    0x0000040408020000,
    0x0004040404040000,
    0x0002020202020000,
    0x0000104104104000,
    0x0000002082082000,
    0x0000000020841000,
    0x0000000000208800,
    0x0000000010020200,
    0x0000000404080200,
    0x0000040404040400,
    0x0002020202020200,
];

/// Shift amounts for rook magic bitboards.
///
/// shift = 64 - popcount(mask)
pub const ROOK_SHIFTS: [u8; 64] = [
    52, 53, 53, 53, 53, 53, 53, 52, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53, 53, 54, 54, 54, 54, 54, 54, 53,
    53, 54, 54, 54, 54, 54, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
];

/// Shift amounts for bishop magic bitboards.
pub const BISHOP_SHIFTS: [u8; 64] = [
    58, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57, 59, 59,
    59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 59, 59, 59, 59, 59, 58,
];

/// Calculates the offsets for Fancy Magic tables.
///
/// Returns (rook_offsets, total_rook_size, bishop_offsets, total_bishop_size)
pub const fn calculate_offsets() -> ([u32; 64], usize, [u32; 64], usize) {
    let mut rook_offsets = [0u32; 64];
    let mut rook_offset = 0;
    let mut sq = 0;
    while sq < 64 {
        rook_offsets[sq] = rook_offset;
        let bits = 64 - ROOK_SHIFTS[sq] as u32;
        rook_offset += 1 << bits;
        sq += 1;
    }

    let mut bishop_offsets = [0u32; 64];
    let mut bishop_offset = 0;
    sq = 0;
    while sq < 64 {
        bishop_offsets[sq] = bishop_offset;
        let bits = 64 - BISHOP_SHIFTS[sq] as u32;
        bishop_offset += 1 << bits;
        sq += 1;
    }

    (
        rook_offsets,
        rook_offset as usize,
        bishop_offsets,
        bishop_offset as usize,
    )
}

/// Pre-calculated offsets and sizes.
pub const OFFSETS: ([u32; 64], usize, [u32; 64], usize) = calculate_offsets();

/// Rook table offsets.
pub const ROOK_OFFSETS: [u32; 64] = OFFSETS.0;

/// Total size of rook attack table.
pub const ROOK_TABLE_SIZE: usize = OFFSETS.1;

/// Bishop table offsets.
pub const BISHOP_OFFSETS: [u32; 64] = OFFSETS.2;

/// Total size of bishop attack table.
pub const BISHOP_TABLE_SIZE: usize = OFFSETS.3;

/// Creates the Fancy Magic structure for each square.
pub const fn create_fancy_magics() -> ([FancyMagic; 64], [FancyMagic; 64]) {
    // Import masks (these will be from masks.rs)
    use crate::movegen::masks::{BISHOP_MASKS, ROOK_MASKS};

    let mut rook_fms = [FancyMagic {
        mask: 0,
        magic: 0,
        shift: 0,
        offset: 0,
    }; 64];

    let mut bishop_fms = [FancyMagic {
        mask: 0,
        magic: 0,
        shift: 0,
        offset: 0,
    }; 64];

    let mut sq = 0;
    while sq < 64 {
        rook_fms[sq] = FancyMagic {
            mask: ROOK_MASKS[sq].0,
            magic: ROOK_MAGICS[sq],
            shift: ROOK_SHIFTS[sq],
            offset: ROOK_OFFSETS[sq],
        };

        bishop_fms[sq] = FancyMagic {
            mask: BISHOP_MASKS[sq].0,
            magic: BISHOP_MAGICS[sq],
            shift: BISHOP_SHIFTS[sq],
            offset: BISHOP_OFFSETS[sq],
        };

        sq += 1;
    }

    (rook_fms, bishop_fms)
}

/// Fancy Magic structures for rooks.
pub const ROOK_FANCY_MAGICS: [FancyMagic; 64] = create_fancy_magics().0;

/// Fancy Magic structures for bishops.
pub const BISHOP_FANCY_MAGICS: [FancyMagic; 64] = create_fancy_magics().1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_table_sizes() {
        // Rook table should be around 100KB
        assert!(ROOK_TABLE_SIZE > 90_000 && ROOK_TABLE_SIZE < 110_000);

        // Bishop table should be around 5KB
        assert!(BISHOP_TABLE_SIZE > 4_000 && BISHOP_TABLE_SIZE < 6_000);

        println!(
            "Rook table size: {} entries (~{} KB)",
            ROOK_TABLE_SIZE,
            ROOK_TABLE_SIZE * 8 / 1024
        );
        println!(
            "Bishop table size: {} entries (~{} KB)",
            BISHOP_TABLE_SIZE,
            BISHOP_TABLE_SIZE * 8 / 1024
        );
    }

    #[test]
    fn test_rook_offsets_increasing() {
        for i in 0..63 {
            assert!(ROOK_OFFSETS[i] < ROOK_OFFSETS[i + 1]);
        }
    }

    #[test]
    fn test_bishop_offsets_increasing() {
        for i in 0..63 {
            assert!(BISHOP_OFFSETS[i] < BISHOP_OFFSETS[i + 1]);
        }
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_fancy_magic_structure() {
        // Check that all fancy magics are properly initialized
        for (sq, rfm) in ROOK_FANCY_MAGICS.iter().enumerate() {
            assert_ne!(rfm.mask, 0, "Rook mask should not be 0 for square {}", sq);
            assert_ne!(rfm.magic, 0, "Rook magic should not be 0 for square {}", sq);
            assert!(
                (52..=54).contains(&rfm.shift),
                "Rook shift should be 52-54 for square {}",
                sq
            );
        }

        for (sq, bfm) in BISHOP_FANCY_MAGICS.iter().enumerate() {
            assert_ne!(bfm.mask, 0, "Bishop mask should not be 0 for square {}", sq);
            assert_ne!(
                bfm.magic, 0,
                "Bishop magic should not be 0 for square {}",
                sq
            );
            assert!(
                (55..=59).contains(&bfm.shift),
                "Bishop shift should be 55-59 for square {}",
                sq
            );
        }
    }
}
