use crate::{
    geomath::Point,
    new_piece,
    types::{Movement, Piece, PieceColor},
};
use indexmap::IndexSet;
use sdl3::rect::Rect;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Knight piece
/// (2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2) or (-1, 2)
#[derive(Clone, PartialEq, Debug)]
pub struct Knight {
    color: PieceColor,
    pos: Point<isize>,
    rect: Rect,
}

impl Display for Knight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♘"; // N
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Knight {
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
        Point::new(1, 2)
            .rotations()
            .into_iter()
            .flat_map(|(p, dir)| self.to_movement(p, None, dir))
            .collect()
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

new_piece!(Knight);
