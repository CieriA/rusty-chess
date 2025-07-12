use crate::{
    geomath::Point,
    types::{Bishop, Color, Movement, Piece, Rook},
};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Queen piece
/// It moves and eats like the `Rook` and the `Bishop` combined.
#[derive(Clone, PartialEq, Debug)]
pub struct Queen {
    color: Color,
    pos: Point,
}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♕"; // Q
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Queen {
    #[inline(always)]
    fn color(&self) -> Color {
        self.color
    }
    #[inline(always)]
    fn pos(&self) -> Point {
        self.pos
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
        let rook = Rook::new(self.color, self.pos);
        let bishop = Bishop::new(self.color, self.pos);

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

impl Queen {
    /// Constructor of Queen
    #[inline]
    pub const fn new(color: Color, pos: Point) -> Self {
        Self { color, pos }
    }
}
