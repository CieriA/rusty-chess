use crate::{geomath::Point, new_piece, types::{Bishop, PieceColor, Movement, Piece, Rook}};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};
use sdl3::rect::Rect;
use crate::chessboard::CELL_SIZE;

/// ## Queen piece
/// It moves and eats like the `Rook` and the `Bishop` combined.
#[derive(Clone, PartialEq, Debug)]
pub struct Queen {
    color: PieceColor,
    pos: Point,
    rect: Rect,
}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♕"; // Q
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Queen {
    #[inline(always)]
    fn color(&self) -> PieceColor {
        self.color
    }
    #[inline(always)]
    fn pos(&self) -> Point {
        self.pos
    }
    #[inline(always)]
    fn rect(&self) -> Rect {
        self.rect
    }
    #[inline(always)]
    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    #[inline(always)]
    fn score(&self) -> u8 {
        9
    }
    fn move_set(&self) -> IndexSet<Movement> {
        let rook = Rook::new(self.color, self.pos, CELL_SIZE);
        let bishop = Bishop::new(self.color, self.pos, CELL_SIZE);

        rook.move_set()
            .into_iter()
            .chain(bishop.move_set())
            .collect()
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

new_piece!(Queen);
