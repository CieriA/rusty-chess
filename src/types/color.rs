use crate::chessboard::Board;

/// Enum describing the color of a [`Piece`](super::piece::Piece).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub enum PieceColor {
    Black,
    #[default]
    White,
}
impl PieceColor {
    /// Depending on the turn, returns the index of the first row from the player perspective
    #[inline]
    pub const fn first_row(&self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => Board::SIZE - 1,
        }
    }

    /// Opposite of the current color.
    ///
    /// > Note: This could've been an implementation of the `Not` trait,
    /// > but I preferred not using that trait because seeing the
    /// > `!` operator on a type `Color` can be misinterpreted.
    /// >
    /// > So I decided to implement it as a stand-alone method.
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    #[inline]
    pub const fn sign(&self) -> isize {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }
}
impl From<bool> for PieceColor {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::White } else { Self::Black }
    }
}
impl From<PieceColor> for bool {
    #[inline]
    fn from(value: PieceColor) -> Self {
        matches!(value, PieceColor::White)
    }
}
