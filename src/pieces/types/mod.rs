mod movement;
mod piece;

#[allow(unused_imports)]
pub use movement::*;
#[allow(unused_imports)]
pub use piece::*;

use crate::chessboard::Board;

/// "Team" of a piece
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub(crate) enum Color {
    #[default]
    White,
    Black,
}
impl Color {
    /// Depending on the turn, returns the index of the first row from the player perspective
    #[inline]
    pub(crate) const fn first_row(&self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => Board::SIZE - 1,
        }
    }

    #[inline]
    pub(crate) const fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    #[inline]
    pub(crate) const fn sign(&self) -> isize {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}
impl From<bool> for Color {
    #[inline]
    fn from(value: bool) -> Self {
        if value {
            Self::White
        } else {
            Self::Black
        }
    }
}
impl From<Color> for bool {
    #[inline]
    fn from(value: Color) -> Self {
        matches!(value, Color::White)
    }
}
