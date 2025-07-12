pub(crate) mod color;
pub(crate) mod movement;
pub(crate) mod piece;
#[cfg(test)]
mod tests;

pub(crate) use color::*;
pub(crate) use movement::*;
pub(crate) use piece::{
    bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook, *,
};
