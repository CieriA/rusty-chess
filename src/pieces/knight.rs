use crate::{
    geomath::Point,
    pieces::{Color, Movement, Piece},
};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Knight piece
/// (2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2) or (-1, 2)
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Knight {
    color: Color,
    pos: Point,
}

impl Display for Knight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♘"; // N
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Knight {
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

impl Knight {
    /// Constructor of Knight
    #[inline]
    pub(crate) const fn new(color: Color, pos: Point) -> Self {
        Self { color, pos }
    }
}
