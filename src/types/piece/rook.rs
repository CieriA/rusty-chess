use crate::{
    chessboard::Board,
    geomath::Point,
    new_piece,
    types::{Movement, Piece, PieceColor, PieceState, State},
};
use indexmap::IndexSet;
use sdl3::rect::Rect;
use std::{
    any::Any,
    fmt::{Display, Formatter},
};

/// ## Rook piece
/// It moves and eats in any direction (not diagonally) as far as it doesn't encounter another piece.
#[derive(Clone, PartialEq, Debug)]
pub struct Rook {
    color: PieceColor,
    pos: Point<isize>,
    rect: Rect,
    state: PieceState,
}

impl Display for Rook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = "♖"; // R
        write!(f, "{}", self.to_colored_string(c))
    }
}
impl Piece for Rook {
    fn color(&self) -> PieceColor {
        self.color
    }
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
        self.set_state(PieceState::Already.into());
    }
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    #[inline(always)]
    fn score(&self) -> u8 {
        5
    }
    fn is_state(&self, state: State) -> bool {
        matches!(state, State::PieceState(ps) if ps == self.state)
    }
    fn move_set(&self) -> IndexSet<Movement> {
        (1..Board::SIZE as isize)
            .flat_map(|i| Point::new(0, i).rotations())
            .flat_map(|(point, dir)| self.to_movement(point, None, dir))
            .collect()
    }
    #[inline]
    fn set_state(&mut self, new_state: State) {
        if let State::PieceState(ps) = new_state {
            self.state = ps;
        } else {
            panic!("Invalid rook state");
        }
    }
    #[inline(always)]
    fn clone_box(&self) -> Box<dyn Piece> {
        Box::new(self.clone())
    }
}

new_piece!(Rook, PieceState);
