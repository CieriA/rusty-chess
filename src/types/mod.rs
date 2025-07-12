pub mod color;
pub mod movement;
pub mod piece;
#[cfg(test)]
mod tests;

pub use color::*;
pub use movement::*;
pub use piece::{
    bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, *,
};
