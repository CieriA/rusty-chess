use std::fmt::{Display, Formatter};
use super::types::{Color, Movement, Piece};
use crate::geomath::Point;
use crate::chessboard::Board;
use indexmap::IndexSet;

/// ## Bishop piece
/// It moves and eats, diagonally, in any direction as far as it doesn't encounter another piece.
#[derive(Clone, PartialEq, Debug)]
pub(crate) struct Bishop {
    color: Color,
    pos: Point,
}

impl Display for Bishop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "B";
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Bishop {
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
    fn move_set(&self) -> IndexSet<Movement> {
        (1..Board::SIZE as isize)
            .flat_map(|i| Point::new(i, i).rotations())
            .flat_map(|(point, dir)| 
                self.to_movement(point, None, dir)
            )
            .collect()
    }
    fn do_move(&mut self, mov: Movement) {
        assert_eq!(self.pos, mov.from);
        self.pos = mov.to;
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Bishop {
    /// Constructor of Bishop
    #[inline]
    pub(crate) const fn new(color: Color, pos: Point) -> Self {
        Self { color, pos }
    }
}
