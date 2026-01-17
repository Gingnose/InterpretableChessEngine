//! InterpretableChessEngine - An interpretable chess engine using graph theory
//!
//! This engine aims to:
//! - Explain why moves are good in human-understandable terms
//! - Generalize to new piece types without manual tuning
//! - Use graph-theoretic evaluation instead of black-box neural networks
//!
//! # Board Size Flexibility
//!
//! The engine supports arbitrary board sizes through generic parameters:
//!
//! ```
//! use interpretable_chess_engine::core::{BoardGeometry, Coord, StandardBoard};
//!
//! // Standard 8x8 chess
//! let e4 = Coord::new(4, 3);
//! assert!(StandardBoard::is_valid(&e4));
//!
//! // Custom 10x10 board
//! type LargeBoard = BoardGeometry<10, 10>;
//! let j10 = Coord::new(9, 9);
//! assert!(LargeBoard::is_valid(&j10));
//! ```

pub mod core;
pub mod eval;
pub mod graph;
pub mod movegen;
pub mod search;
pub mod threats;
pub mod uci;
pub mod variants;

// Re-export core types for convenience
pub use core::{BoardGeometry, Color, Coord, Delta, StandardBoard};
