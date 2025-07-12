use crate::{
    geomath::{Point, rotation::Direction},
    types::*,
};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// If the inside `bool` is `false`, it means that the `King` has never been moved.
/// ## King piece
/// It moves and eats in any direction, diagonally and non-diagonally, by 1 square.
///
/// It can't go in a square where it could get eaten.
///
/// If it has never been moved, the King can do a castle with one of the Rook,
/// if it also has never been moved,
/// moving the King (2, 0) (H1 rook) or (-2, 0) (A1 rook)
/// and the Rook next to the king in the opposite direction.
/// #### Win conditions
/// **Check**: When the King could get eaten the next turn if not moved.
/// - When in **check**, you must protect the King someway.
///
/// **Checkmate**: When the King is in **Check** and there's no way to get it out of **Check**
/// (it is, it can't be protected nor moved in a square where it isn't in **check**).
/// - If it happens, the player lose.
///
/// **Stalemate**: The king is the only _moveable_ piece, it is **not** in **check**
/// but all the squares it could go would put it in **check**.
/// - If it happens, it's a draw.
#[derive(Clone, PartialEq, Debug)]
pub struct King {
    color: Color,
    pos: Point,
    state: PieceState,
}

impl Display for King {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♔"; // K
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for King {
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
        self.set_state(PieceState::Already.into());
    }
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    #[inline(always)]
    fn score(&self) -> u8 {
        unreachable!()
    } // should not be called
    fn is_state(&self, state: State) -> bool {
        matches!(state, State::PieceState(ps) if ps == self.state)
    }
    fn move_set(&self) -> IndexSet<Movement> {
        Point::all_around(1)
            .into_iter()
            .map(|(point, dir)| self.to_movement(point, None, dir))
            .chain([
                self.to_movement(
                    Point::new(-2, 0),
                    Some(SpecialMove::LongCastle),
                    Some(Direction::Left),
                ),
                self.to_movement(
                    Point::new(2, 0),
                    Some(SpecialMove::ShortCastle),
                    Some(Direction::Right),
                ),
            ])
            .flatten()
            .collect()
    }
    #[inline]
    fn set_state(&mut self, new_state: State) {
        if let State::PieceState(ps) = new_state {
            self.state = ps;
        } else {
            panic!("Invalid king state");
        }
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

impl King {
    /// Constructor of King
    #[inline]
    pub fn new(color: Color, pos: Point) -> Self {
        Self {
            color,
            pos,
            state: PieceState::default(),
        }
    }
}
