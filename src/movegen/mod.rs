//! Move generation module.

pub mod attacks;
pub mod bitboard;
pub mod legal_moves;
pub mod magic_constants;
pub mod masks;
pub mod rays;

pub use attacks::{
    bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
};
pub use bitboard::Bitboard64;
pub use legal_moves::{generate_legal_moves, is_in_check, perft, MoveGenerator};
pub use masks::{BISHOP_MASKS, ROOK_MASKS};
pub use rays::{bishop_attacks_slow, blocker_permutations, rook_attacks_slow};
