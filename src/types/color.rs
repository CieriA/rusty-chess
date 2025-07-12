use crate::chessboard::Board;

/// Enum describing the color of a [`Piece`](super::piece::Piece).
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub(crate) enum Color {
    Black,
    #[default]
    White,
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
        if value { Self::White } else { Self::Black }
    }
}
impl From<Color> for bool {
    #[inline]
    fn from(value: Color) -> Self {
        matches!(value, Color::White)
    }
}
