//! Move generation module.

pub mod bitboard;
pub mod masks;
pub mod rays;

pub use bitboard::Bitboard64;
pub use masks::{BISHOP_MASKS, ROOK_MASKS};
pub use rays::{bishop_attacks_slow, blocker_permutations, rook_attacks_slow};
