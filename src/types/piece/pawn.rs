use crate::{
    geomath::{Point, rotation::Direction},
    types::*,
};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Pawn piece
/// It moves by **1 square upwards** normally, or by **2 squares upwards** if it has never been moved.
///
/// It can only eat a piece which is **1 square higher diagonally** than it.
///
/// **En passant**: When an **opponent's Pawn** moves by 2 squares arriving next to yours,
/// your Pawn can move **1 square diagonally** behind that Pawn and eat it.
///
/// When it arrives to the last row, it can **upgrade** to a **Knight / Rook / Bishop / Queen**.
#[derive(Clone, PartialEq, Debug)]
pub struct Pawn {
    color: Color,
    pos: Point,
    pub state: PawnState,
}

impl Display for Pawn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♙"; // P
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Pawn {
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
        1
    }
    #[inline(always)]
    fn is_state(&self, state: State) -> bool {
        matches!(state, State::PawnState(ps) if ps == self.state)
    }
    fn move_set(&self) -> IndexSet<Movement> {
        [
            self.to_movement(
                Point::new(0, 1),
                Some(SpecialMove::CannotEat),
                Some(Direction::Up),
            ),
            self.to_movement(
                Point::new(0, 2),
                Some(SpecialMove::DoublePawn),
                Some(Direction::Up),
            ),
            self.to_movement(
                Point::new(1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpRight),
            ),
            self.to_movement(
                Point::new(-1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpLeft),
            ),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    #[inline]
    fn set_state(&mut self, new_state: State) {
        if let State::PawnState(ps) = new_state {
            self.state = ps;
        } else {
            panic!("Invalid pawn state");
        }
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl Pawn {
    /// Constructor of Pawn
    #[inline]
    pub fn new(color: Color, pos: Point) -> Self {
        Self {
            color,
            pos,
            state: PawnState::default(),
        }
    }
}
