//! InterpretableChessEngine - An interpretable chess engine using graph theory
//!
//! This engine aims to:
//! - Explain why moves are good in human-understandable terms
//! - Generalize to new piece types without manual tuning
//! - Use graph-theoretic evaluation instead of black-box neural networks

pub mod core;
pub mod eval;
pub mod graph;
pub mod movegen;
pub mod search;
pub mod threats;
pub mod uci;
pub mod variants;

// Re-export core types for convenience
pub use core::{Color, Delta, Square};
