use std::fmt::{Debug, Display};
use colored::{ColoredString, Colorize};
use crate::chessboard::Board;
use crate::geomath::Point;
use crate::pieces::{
    types::{Color, Movement},
    bishop::Bishop,
    knight::Knight,
    rook::Rook,
    queen::Queen,
    king::King,
};
use crate::geomath::rotation::Direction;
use super::movement::SpecialMove;
use indexmap::IndexSet;

/// A trait representing a Chess Piece.
pub(crate) trait Piece: Display + Debug {
    #[must_use]
    fn color(&self) -> Color;
    #[must_use]
    fn pos(&self) -> Point;
    
    fn set_pos(&mut self, pos: Point);
    #[inline(always)]
    #[must_use]
    fn set_pos_upgrade(&mut self, pos: Point) -> Option<Box<dyn Piece>> {
        self.set_pos(pos);
        None
    }
    #[inline(always)]
    #[must_use]
    fn is_king(&self) -> bool { false }
    #[inline(always)]
    #[must_use]
    fn is_pawn(&self) -> bool { false }
    
    #[must_use]
    fn score(&self) -> u8;
    
    
    /// Returns `true` if the piece has the given state or `false` otherwise.
    /// If the piece has no state at all, `true` is returned.
    #[must_use]
    #[allow(unused_variables)]
    fn is_state(&self, state: State) -> bool { true }
    /// Returns the actual color if the direction is important for the piece,
    /// or Color::default() if it is not. 
    #[inline(always)]
    #[must_use]
    fn color_if_has_direction(&self) -> Color {
        Color::default()
    }
    /// From an offset (and Self) returns a new Movement.
    fn to_movement(
        &self, offset: Point, special: Option<SpecialMove>, direction: Option<Direction>
    ) -> Result<Movement, ()> {
        let to = self.pos() + (offset * self.color_if_has_direction().sign());
        if !Board::in_bounds(to) {
            return Err(());
        }
        Ok(Movement::new(
            self.pos(),
            to,
            special,
            direction,
        ))
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
    /// Given a &str, returns the &str colored based on self.color().
    #[inline]
    #[must_use]
    fn to_colored_string(&self, c: &str) -> ColoredString {
        if self.color().into() { c.bright_white() } else { c.bright_blue() }
    }
    #[must_use]
    fn clone_box(&self) -> Box<dyn Piece>;
}
pub(crate) fn placement(x: isize, color: Color) -> Box<dyn Piece> {
    let pos = Point::new(x, color.first_row() as isize);
    match x {
        0 | 7 => Box::new(Rook::new(color, pos)),
        1 | 6 => Box::new(Knight::new(color, pos)),
        2 | 5 => Box::new(Bishop::new(color, pos)),
        3 => Box::new(Queen::new(color, pos)),
        4 => Box::new(King::new(color, pos)),
        _ => panic!("Impossible case"),
    }
}
pub(crate) fn piece_from_char(c: char, color: Color, pos: Point) -> Box<dyn Piece> { // TODO tests
    match c.to_ascii_uppercase() {
        'B' => Box::new(Bishop::new(color, pos)),
        'N' => Box::new(Knight::new(color, pos)),
        'R' => Box::new(Rook::new(color, pos)),
        'Q' => Box::new(Queen::new(color, pos)),
        _ => panic!("Invalid char"),
    }
}

/// Wrapper of all the possible states
pub(crate) enum State {
    PawnState(PawnState),
    PieceState(PieceState),
}
impl From<PawnState> for State {
    #[inline(always)]
    fn from(value: PawnState) -> Self {
        Self::PawnState(value)
    }
}
impl From<PieceState> for State {
    #[inline(always)]
    fn from(value: PieceState) -> Self {
        Self::PieceState(value)
    }
}

/// A pawn can be not yet moved, just moved or already moved.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub(crate) enum PawnState {
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
pub(crate) enum PieceState {
    /// Not yet moved: can do a special move.
    #[default]
    NotYet,

    /// Already moved: cannot do a special move.
    Already,
}
