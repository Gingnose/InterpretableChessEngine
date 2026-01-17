//! Core data structures for the chess engine.

pub mod board;
pub mod board_geometry;
pub mod color;
pub mod coord;
pub mod delta;
pub mod gamestate;
pub mod moves;
pub mod piece;

pub use board::Board;
pub use board_geometry::{BoardGeometry, StandardBoard};
pub use color::Color;
pub use coord::Coord;
pub use delta::Delta;
pub use gamestate::{CastlingRights, GameState};
pub use moves::{Move, MoveFlags};
pub use piece::{MovementType, Piece, PieceDefinition, PieceType};
