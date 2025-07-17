use crate::types::{Movement, Piece, PieceColor};
use crate::{chessboard::Board, geomath::Point, new_piece};
use indexmap::IndexSet;
use sdl3::rect::Rect;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Bishop piece
/// It moves and eats, diagonally, in any direction as far as it doesn't encounter another piece.
#[derive(Clone, PartialEq, Debug)]
pub struct Bishop {
    color: PieceColor,
    pos: Point<isize>,
    rect: Rect,
}

impl Display for Bishop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♗"; // B
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Bishop {
    #[inline(always)]
    fn color(&self) -> PieceColor {
        self.color
    }
    #[inline(always)]
    fn pos(&self) -> Point<isize> {
        self.pos
    }
    #[inline(always)]
    fn rect(&self) -> Rect {
        self.rect
    }
    #[inline(always)]
    fn set_pos(&mut self, pos: Point<isize>) {
        self.pos = pos;
    }
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    #[inline(always)]
    fn score(&self) -> u8 {
        3
    }
    fn move_set(&self) -> IndexSet<Movement> {
        (1..Board::SIZE as isize)
            .flat_map(|i| Point::new(i, i).rotations())
            .flat_map(|(point, dir)| self.to_movement(point, None, dir))
            .collect()
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

new_piece!(Bishop);
