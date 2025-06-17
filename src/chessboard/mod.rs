//! 8*8 classic Chessboard
#[cfg(test)]
mod tests;

use crate::{
    pieces::{types::{Piece, Movement, SpecialMove, Color}, prelude::*},
    geomath::{Point, rotation::Direction},
};
use colored::Colorize;
use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
    fmt::Display,
    slice::Iter,
};
use indexmap::IndexSet;
use crate::pieces::types;

pub(crate) type Square = Option<Box<dyn Piece>>; // Change to piece
pub(crate) type Row = [Square; Board::SIZE];
pub(crate) type Grid = [Row; Board::SIZE];

#[derive(Debug)]
pub(crate) struct Board(Grid);

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut board = Board::empty();
        for (y, row) in self.0.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if let Some(piece) = square {
                    board[Point::new(x as isize, y as isize)] = Some(piece.clone_box());
                }
            }
        }
        board
    }
}
impl Default for Board {
    fn default() -> Self {
        let mut board = Self::empty();
        
        // White pawns
        for (x, cell) in board[1].iter_mut().enumerate() {
            *cell = Some(Box::new(
                Pawn::new(Color::White, Point::new(x as isize, 1))
            ));
        }
        // Black pawns
        for (x, cell) in board[Self::SIZE - 2].iter_mut().enumerate() {
            *cell = Some(Box::new(
                Pawn::new(Color::Black, Point::new(x as isize, Self::SIZE as isize - 2))
            ));
        }
        
        // Other pieces
        for x in 0..Self::SIZE as isize {
            board[Point::new(x, 0)] =
                Some(types::placement(x, Color::White));
            
            board[Point::new(x, Self::SIZE as isize - 1)] = 
                Some(types::placement(x, Color::Black));
        }
        
        board
    }
}
impl Index<Point> for Board {
    type Output = Square;
    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        assert!(
            Self::in_bounds(index),
            "(x, y): {}", index
        );
        &self.0[index.y as usize][index.x as usize]
    }
}
impl IndexMut<Point> for Board {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        assert!(
            Self::in_bounds(index),
            "(x, y): {}", index
        );
        &mut self.0[index.y as usize][index.x as usize]
    }
}
impl Index<usize> for Board {
    type Output = Row;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Board {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.0.iter().rev().enumerate() {
            write!(f, "{} ", Board::SIZE - i)?;
            for (j, cell) in row.iter().enumerate() {
                let colored = match cell {
                    Some(piece) if (i + j) % 2 == 0 => format!(" {} ", piece).normal(),
                    Some(piece) => format!(" {} ", piece).on_cyan(),
                    None if (i + j) % 2 == 0 => "   ".normal(),
                    None => "   ".on_cyan(),
                };
                write!(f, "{}", colored)?;
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        let chs = ('a'..='h').collect::<Vec<char>>();
        for c in chs.iter().take(Board::SIZE) { // Just to be sure: take
            write!(f, " {} ", c)?;
        }
        Ok(())
    }
}

impl Board {
    pub(crate) const SIZE: usize = 8;
    /// Constructor of `Board` returning all of its squares as `None`
    #[inline]
    pub(crate) fn empty() -> Self {
        Self(Grid::default())
    }
    /// Checks if a `Point` is inside the Board.
    #[inline]
    pub(crate) const fn in_bounds(point: Point) -> bool {
        point.x < Self::SIZE as isize &&
        point.y < Self::SIZE as isize &&
        point.x >= 0 &&
        point.y >= 0
    }
    #[inline]
    pub(crate) fn get(&self, point: Point) -> Option<&Square> {
        if !Board::in_bounds(point) {
            None
        } else {
            Some(&self[point])
        }
    }
    #[inline]
    pub(crate) fn iter(&self) -> Iter<'_, Row> {
        self.0.iter()
    }
    /// From the normal `.move_set()`, returns only the possible moves,
    /// filtering:
    /// - Collisions (for bishops, pawns, rooks and queens)     **(1° .filter())**
    /// - Impossible SpecialMoves                               **(2° .filter())**
    /// 
    /// The `from` parameter is the `Movement.from` field.
    pub(crate) fn filtered_move_set(&self, from: Point) -> IndexSet<Movement> {
        let mut ignored: HashSet<Direction> = HashSet::new();
        
        // .unwrap() checks that the piece exists
        let piece = self[from].as_ref().unwrap();

        piece
            .move_set()
            .into_iter()
            .filter(|mov| {
                let Some(direction) = mov.direction else {
                    return self[mov.to].as_ref().is_none_or(|new_piece| 
                        new_piece.color() != piece.color()
                    );
                };
                if ignored.contains(&direction) {
                    return false;
                }
                match &self[mov.to] {
                    Some(new_piece) => {
                        ignored.insert(direction);
                        new_piece.color() != piece.color()
                    }
                    None => !ignored.contains(&direction),
                }
            })
            .filter(|mov| {
                let Some(ref special) = mov.special else {
                    return true;
                };
                match special {
                    SpecialMove::DoublePawn => 
                        piece.is_state(State::PawnState(PawnState::NotYet)),
                    
                    SpecialMove::PawnEat => {
                        let to = mov.to;
                        let new_piece = match &self[to] {
                            Some(new_piece) => new_piece,
                            None => {
                                let Some(eat_square) =
                                    self.get(to - Point::new(0, piece.color().sign())) else {
                                    return false;
                                };
                                let Some(new_piece) = 
                                    eat_square else {
                                        return false;
                                };
                                if !new_piece.is_state(State::PawnState(PawnState::JustDouble)) {
                                    return false;
                                }
                                new_piece
                            }
                        };
                        new_piece.color() != piece.color()
                    },
                    
                    SpecialMove::ShortCastle => {
                        let rook_pos = mov.from + Point::new(3, 0);
                        let Some(rook) = self[rook_pos].as_ref() else {
                            return false;
                        };

                        let start = mov.from.x as usize + 1;
                        let end = rook_pos.x as usize;
                        let slice = 
                            &self[piece.color().first_row()][start..end];
                        
                        slice.iter().all(Option::is_none) &&
                        piece.is_state(State::PieceState(PieceState::NotYet)) &&
                        rook.is_state(State::PieceState(PieceState::NotYet))
                    }
                    
                    SpecialMove::LongCastle => {
                        let rook_pos = mov.from + Point::new(-4, 0);
                        let Some(rook) = self[rook_pos].as_ref() else {
                            return false;
                        };
                        let start = rook_pos.x as usize + 1;
                        let end = mov.from.x as usize;
                        let slice =
                            &self[piece.color().first_row()][start..end];
                        
                        slice.iter().all(Option::is_none) &&
                        piece.is_state(State::PieceState(PieceState::NotYet)) &&
                        rook.is_state(State::PieceState(PieceState::NotYet))
                    }
                }
            })
            .collect()
    }
    /// Returns all the moves a player (`color`) can do.
    fn all_moves(&self, color: Color) -> HashSet<Movement> {
        let mut set = HashSet::new();
        for (y, row) in self.iter().enumerate() {
            for (x, square) in row.iter().enumerate() { 
                if !square.as_ref().is_some_and(|p| p.color() == color) {
                    continue;
                }
                set.extend(self.filtered_move_set(Point::new(x as isize, y as isize)));
            }
        }
        set
    }
    /// Do a move, checking if it is possible.
    ///
    /// Returns `true` if all went right, `false` otherwise
    pub(crate) fn do_move(&mut self, mov: Movement) -> bool {
        if !self.filtered_move_set(mov.from).contains(&mov) {
            false
        } else {
            self.apply_move(mov);
            true
        }
    }
    /// Without checking errors, move a piece.
    fn apply_move(&mut self, mov: Movement) {
        let mut piece = self[mov.from].take().unwrap(); // .unwrap() to ensure the piece exists
        let upgraded = piece.set_pos_upgrade(mov.to);
        if let Some(upgraded) = upgraded {
            self[mov.to] = Some(upgraded);
        }
        self[mov.to] = Some(piece);
        match mov.special {
            Some(SpecialMove::ShortCastle) => {
                let mut rook = self[mov.from + Point::new(3, 0)].take().unwrap();
                let new_pos = mov.to - Point::new(1, 0);
                
                rook.set_pos(new_pos);
                self[new_pos] = Some(rook);
            }
            Some(SpecialMove::LongCastle) => {
                let mut rook = self[mov.from - Point::new(4, 0)].take().unwrap();
                let new_pos = mov.to + Point::new(1, 0);
                
                rook.set_pos(new_pos);
                self[new_pos] = Some(rook);
            }
            Some(SpecialMove::PawnEat) if self[mov.to].is_none() => {
                let color = self[mov.from].as_ref().unwrap().color();
                let new_pos = mov.to - Point::new(0, color.sign());
                
                self[new_pos].take();
            }
            _ => {}
        }
    }
    /// Returns the coordinates of the King of the given color.
    /// TODO: maybe change this to returning a reference
    fn find_king(&self, color: Color) -> Point { // TODO tests
        for (y, row) in self.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if square
                    .as_ref()
                    .is_some_and(|piece| 
                        piece.is_king() && piece.color() == color
                    ) 
                {
                    return Point::new(x as isize, y as isize);
                }
            }
        }
        unreachable!("There should be a King");
    }
    /// Is the `color` player in check?
    pub(crate) fn check(&self, color: Color) -> Option<Movement> { // TODO tests
        let king_pos = self.find_king(color);
        self
            .all_moves(color.opposite())
            .into_iter()
            .find(|mov| mov.to == king_pos)
    }
    /// Can the player block the Check moving a piece?
    /// `color` is the current player
    pub(crate) fn is_check_stoppable(&self, color: Color) -> bool {
        let check_move = self.check(color).unwrap();
        false
    }
    pub(crate) fn checks_around(&self, color: Color) -> bool { // TODO tests
        self
            .filtered_move_set(self.find_king(color))
            .into_iter()
            .all(|mov| {
                let mut new_board = self.clone();
                new_board.do_move(mov);
                new_board.check(color).is_some()
            })
        
    }
    pub(crate) fn checkmate(&self, color: Color) -> bool { // TODO tests
        // Return is check and all around check
        // TODO check for possible blocks of the check
        let _king_pos = self.find_king(color);
        
        self.check(color).is_some() && self.checks_around(color)
    }
    #[inline(always)]
    pub(crate) fn stalemate(&self, color: Color) -> bool { // TODO tests
        // No moves available and not check
        self.check(color).is_none() && self.all_moves(color).is_empty()
    }
}
