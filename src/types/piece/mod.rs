pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::{
    chessboard::Board,
    geomath::{Point, rotation::Direction},
    types::*,
};
use colored::{ColoredString, Colorize};
use indexmap::IndexSet;
use std::{
    any::Any,
    fmt::{Debug, Display},
};

/// A trait representing a Chess Piece.
pub trait Piece: Display + Debug + Any {
    /// Color of a given piece
    ///
    /// > This enforces the definition of a type that implements `Piece`
    /// > to have a `color` field, returned by this function.
    /// >
    /// > Typical `Piece::color` implementations look like
    /// > ```Rust
    /// > #[inline]
    /// > fn color(&self) -> Point {
    /// >     self.color
    /// > }
    /// > ```
    #[must_use]
    fn color(&self) -> Color;
    /// Position of a given piece on the board
    ///
    /// > This enforces the definition of a type that implements `Piece`
    /// > to have a `pos` field, returned by this function.
    /// >
    /// > Typical `Piece::pos` implementations look like
    /// > ```Rust
    /// > #[inline]
    /// > fn pos(&self) -> Point {
    /// >     self.pos
    /// > }
    /// > ```
    #[must_use]
    fn pos(&self) -> Point;

    fn set_pos(&mut self, pos: Point);
    /// Returns self as `&dyn Any`.
    ///
    /// This method exists because a trait object is not [`Sized`],
    /// and so you can't cast `Box<dyn Piece>` to `&dyn Any` using `as`.
    ///
    /// As a result, this method serve as a helper to cast `Box<dyn Piece>` to `&dyn Any`.
    ///
    /// > Typical `Piece::as_any` implementations look like
    /// > ```Rust
    /// > #[inline]
    /// > fn as_any(&self) -> &dyn Any {
    /// >     self as &dyn Any
    /// > }
    fn as_any(&self) -> &dyn Any;

    /// In chess, each piece has a value. This method returns that value.
    #[must_use]
    fn score(&self) -> u8;

    /// Returns `true` if the piece has the given state or `false` otherwise.
    /// If the piece has no state at all, `true` is returned.
    #[must_use]
    #[allow(unused_variables)]
    fn is_state(&self, state: State) -> bool {
        true
    }
    /// Returns the actual color if the direction is important for the piece,
    /// or Color::default() if it is not.
    #[inline]
    #[must_use]
    fn color_if_has_direction(&self) -> Color {
        if self.as_any().is::<Pawn>() {
            self.color()
        } else {
            Color::default()
        }
    }
    /// From an offset (and Self) returns a new Movement.
    fn to_movement(
        &self,
        offset: Point,
        special: Option<SpecialMove>,
        direction: Option<Direction>,
    ) -> Option<Movement> {
        let to = self.pos() + (offset * self.color_if_has_direction().sign());

        Board::in_bounds(to).then(|| Movement::new(self.pos(), to, special, direction))
    }

    /// An HashSet of all the possible moves of a piece,
    /// not considering collisions.
    /// Use the Board for that.
    #[must_use]
    fn move_set(&self) -> IndexSet<Movement>;

    /// Sets the state of the piece to something else, if it has it.
    ///
    /// Panics if the state is not of tha valid type.
    #[allow(unused_variables)]
    fn set_state(&mut self, state: State) {}
    /// Given a &str, returns the &str colored based on [`Piece::color`].
    #[inline]
    #[must_use]
    fn to_colored_string(&self, c: &str) -> ColoredString {
        if self.color().into() {
            c.bright_white()
        } else {
            c.bright_blue()
        }
    }

    /// Clones self into a `Box<dyn Piece>`.
    ///
    /// This method exists because the [`Clone`] trait
    /// is not [`dyn compatible`](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility),
    /// meaning you can't call [`Clone::clone`] on a `Box<dyn Piece>` or `&dyn Piece`.
    ///
    /// As a result, every type that implements [`Piece`] must also implement [`Clone`],
    /// even though this constraint cannot be expressed directly in the [`Piece`] definition.
    ///
    /// > Typical `Piece::clone_box` implementations look like
    /// > ```Rust
    /// > #[inline]
    /// > fn clone_box(&self) -> Box<dyn Piece> {
    /// >     Box::new(self.clone())
    /// > }
    /// > ```
    #[must_use]
    fn clone_box(&self) -> Box<dyn Piece>;
}
/// Given an offset from the start of the board, returns
/// the correct piece which should be in that spot.
pub fn placement(x: isize, color: Color) -> Option<Box<dyn Piece>> {
    let pos = Point::new(x, color.first_row() as isize);
    match x {
        0 | 7 => Some(Box::new(Rook::new(color, pos))),
        1 | 6 => Some(Box::new(Knight::new(color, pos))),
        2 | 5 => Some(Box::new(Bishop::new(color, pos))),
        3 => Some(Box::new(Queen::new(color, pos))),
        4 => Some(Box::new(King::new(color, pos))),
        _ => None,
    }
}
/// Returns a `Box<dyn Piece>` from its [`char`] representation.
///
/// Only works for [`Bishop`], [`Knight`], [`Rook`] and [`Queen`], which are the
/// pieces that a [`Pawn`] can promote to.
///
/// All other pieces will make this
/// associated function panic.
pub fn piece_from_char(c: char, color: Color, pos: Point) -> Option<Box<dyn Piece>> {
    match c.to_ascii_uppercase() {
        'B' => Some(Box::new(Bishop::new(color, pos))),
        'N' => Some(Box::new(Knight::new(color, pos))),
        'R' => Some(Box::new(Rook::new(color, pos))),
        'Q' => Some(Box::new(Queen::new(color, pos))),
        _ => None,
    }
}

/// Wrapper of all the possible states
pub enum State {
    PawnState(PawnState),
    PieceState(PieceState),
}
impl From<PawnState> for State {
    #[inline]
    fn from(value: PawnState) -> Self {
        Self::PawnState(value)
    }
}
impl From<PieceState> for State {
    #[inline]
    fn from(value: PieceState) -> Self {
        Self::PieceState(value)
    }
}

/// A pawn can be not yet moved, just moved or already moved.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PawnState {
    /// The pawn hasn't been moved and can do a double move.
    #[default]
    NotYet,

    /// The pawn has just done a double move and
    /// can suffer from En Passant.
    JustDouble,

    /// The pawn has already done a double move,
    /// cannot do it anymore and cannot suffer an En Passant.
    Already,
}

/// A piece like the King, to do a castle, mustn't have been moved.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PieceState {
    /// Not yet moved: can do a special move.
    #[default]
    NotYet,

    /// Already moved: cannot do a special move.
    Already,
}
