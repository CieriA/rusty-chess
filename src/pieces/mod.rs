pub(crate) mod bishop;
pub(crate) mod king;
pub(crate) mod knight;
pub(crate) mod pawn;
pub(crate) mod queen;
pub(crate) mod rook;
pub(crate) mod types;
#[cfg(test)]
mod tests;

pub(crate) mod prelude {
    #![allow(unused_imports)]
    pub(crate) use super::{
        bishop::Bishop,
        king::King,
        knight::Knight,
        pawn::Pawn,
        queen::Queen,
        rook::Rook,
        types::*,
    };
}
