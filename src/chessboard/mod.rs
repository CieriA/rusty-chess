//! 8*8 classic Chessboard

use {
    crate::{geomath::Point, types::*},
    colored::Colorize as _,
    indexmap::IndexSet,
    std::{
        collections::HashSet,
        fmt::{self, Display},
        ops::{Index, IndexMut},
    },
};

#[cfg(test)]
mod tests;

pub type Square = Option<Box<dyn Piece>>;
pub type Row = [Square; Board::SIZE];
pub type Grid = [Row; Board::SIZE];

#[derive(Debug)]
pub struct Board(Grid);

impl Clone for Board {
    fn clone(&self) -> Self {
        let mut board = Board::empty();
        for (y, row) in self.iter().enumerate() {
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

        for color in [Color::White, Color::Black] {
            for x in 0..Board::SIZE as isize {
                let coord = Point::new(x, color.second_row() as isize);
                board[coord] = Some(Box::new(Pawn::new(color, coord)));

                board[Point::new(x, color.first_row() as isize)] = placement(x, color);
            }
        }

        board
    }
}
impl Index<Point> for Board {
    type Output = Square;
    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        assert!(Self::in_bounds(index), "(x, y): {index}");
        &self.0[index.y as usize][index.x as usize]
    }
}
impl IndexMut<Point> for Board {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        assert!(Self::in_bounds(index), "(x, y): {index}");
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.0.iter().rev().enumerate() {
            write!(f, "{} ", Board::SIZE - i)?;
            for (j, cell) in row.iter().enumerate() {
                let colored = match cell {
                    Some(piece) if (i + j) % 2 == 0 => format!(" {piece} ").normal(),
                    Some(piece) => format!(" {piece} ").on_cyan(),
                    None if (i + j) % 2 == 0 => "   ".normal(),
                    None => "   ".on_cyan(),
                };
                write!(f, "{colored}")?;
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        let chs = ('a'..='h').collect::<Vec<char>>();
        for c in chs.iter().take(Board::SIZE) {
            // Just to be sure: take
            write!(f, " {c} ")?;
        }
        Ok(())
    }
}

impl Board {
    pub const SIZE: usize = 8;
    /// Constructor of `Board` returning all of its squares as `None`
    #[inline]
    pub fn empty() -> Self {
        Self(Grid::default())
    }
    /// Checks if a `Point` is inside the Board.
    #[inline]
    pub const fn in_bounds(point: Point) -> bool {
        point.x < Self::SIZE as isize
            && point.y < Self::SIZE as isize
            && point.x >= 0
            && point.y >= 0
    }
    #[inline]
    pub fn get(&self, point: Point) -> Option<&Square> {
        Board::in_bounds(point).then_some(&self[point])
    }
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Row> {
        self.0.iter()
    }
    /// From the normal `.move_set()`, returns only the possible moves,
    /// filtering:
    /// - **(1° .filter())**: Collisions (for bishops, pawns, rooks and queens)
    /// - **(2° .filter())**: Impossible SpecialMoves
    ///
    /// The `from` parameter is the `Movement.from` field.
    pub fn filtered_move_set(&self, from: Point) -> IndexSet<Movement> {
        let mut ignored = HashSet::new();

        // .unwrap() checks that the piece exists
        let piece = self[from].as_ref().unwrap();

        piece
            .move_set()
            .into_iter()
            .filter(|mov| {
                let Some(direction) = mov.direction else {
                    return self[mov.to]
                        .as_ref()
                        .is_none_or(|new_piece| new_piece.color() != piece.color());
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
                    SpecialMove::CannotEat => self[mov.to].is_none(),

                    SpecialMove::DoublePawn => {
                        assert_eq!(mov.from.x, mov.to.x);

                        let range = if piece.color().into() { 1..3 } else { -2..0 };

                        range
                            .map(|y| mov.from + Point::new(0, y))
                            .all(|p| self[p].is_none())
                            && piece.is_state(State::PawnState(PawnState::NotYet))
                    }

                    SpecialMove::PawnEat => {
                        let to = mov.to;
                        let new_piece = match &self[to] {
                            Some(new_piece) => new_piece,
                            None => {
                                let Some(Some(new_piece)) =
                                    self.get(to - Point::new(0, piece.color().sign()))
                                else {
                                    return false;
                                };
                                if !new_piece.is_state(State::PawnState(PawnState::JustDouble)) {
                                    return false;
                                }
                                new_piece
                            }
                        };
                        new_piece.color() != piece.color()
                    }

                    SpecialMove::ShortCastle => {
                        let rook_pos = mov.from + Point::new(3, 0);
                        if !Board::in_bounds(rook_pos) {
                            return false;
                        }
                        let Some(rook) = self[rook_pos].as_ref() else {
                            return false;
                        };

                        let start = mov.from.x as usize + 1;
                        let end = rook_pos.x as usize;

                        let slice = &self[piece.color().first_row()][start..end];

                        piece.is_state(State::PieceState(PieceState::NotYet))
                            && rook.is_state(State::PieceState(PieceState::NotYet))
                            && slice.iter().all(Option::is_none)
                            && !self
                                .all_moves(piece.color().opposite())
                                .into_iter()
                                .any(|mov| {
                                    (0..2)
                                        .map(|i| piece.pos() + Point::new(i, 0))
                                        .any(|p| mov.to == p)
                                })
                    }

                    SpecialMove::LongCastle => {
                        let rook_pos = mov.from + Point::new(-4, 0);
                        if !Board::in_bounds(rook_pos) {
                            return false;
                        }
                        let Some(rook) = self[rook_pos].as_ref() else {
                            return false;
                        };
                        let start = rook_pos.x as usize + 1;
                        let end = mov.from.x as usize;
                        let slice = &self[piece.color().first_row()][start..end];

                        piece.is_state(State::PieceState(PieceState::NotYet))
                            && rook.is_state(State::PieceState(PieceState::NotYet))
                            && slice.iter().all(Option::is_none)
                            && !self
                                .all_moves(piece.color().opposite())
                                .into_iter()
                                .any(|mov| {
                                    (-2..0)
                                        .map(|i| piece.pos() + Point::new(i, 0))
                                        .any(|p| mov.to == p)
                                })
                    }
                }
            })
            .collect()
    }
    /// Coordinates of all pieces on the board
    pub fn all_pieces(&self) -> HashSet<Point> {
        let mut set = HashSet::new();
        for (y, row) in self.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if square.is_some() {
                    set.insert(Point::new(x as isize, y as isize));
                }
            }
        }
        set
    }
    /// Coordinates of all pieces with a given color on the board
    fn all_color_pieces(&self, color: Color) -> HashSet<Point> {
        self.all_pieces()
            .into_iter()
            // by using `.unwrap()` instead of `.is_some_and()` I assure `.all_pieces()` works too,
            // and if it doesn't this fn will panic,
            // but maybe `.is_some_and()` was more safe and better in this case.
            .filter(|point| self[*point].as_ref().unwrap().color() == color)
            .collect()
    }
    /// Returns all the moves a player (`color`) can do.
    pub fn all_moves(&self, color: Color) -> HashSet<Movement> {
        let mut set = HashSet::new();
        for coord in self.all_color_pieces(color) {
            if self[coord]
                .as_ref()
                .is_some_and(|piece| piece.color() == color)
            {
                set.extend(self.filtered_move_set(Point::new(coord.x, coord.y)));
            }
        }
        set
    }
    #[inline]
    pub fn is_promoting(&self, mov: &Movement) -> bool {
        self[mov.from].as_ref().is_some_and(|piece| {
            piece.as_any().is::<Pawn>() && piece.color().opposite().first_row() as isize == mov.to.y
        })
    }
    /// Returns `Some` if the move is an eating move, none otherwise
    /// `Some` contains the points (`usize`) added to the score to the `Color` player.
    ///
    /// Also change the state of all pawns already moved.
    ///
    /// `promoted` is the piece to which the pawn should promote
    /// input to the user to promote the pawn or not.
    pub fn do_move(
        &mut self,
        mov: &Movement,
        promoted: Option<Box<dyn Piece>>,
    ) -> Option<(f64, Color)> {
        let piece = self[mov.from].as_ref().unwrap();
        let color = piece.color();

        // score
        let res = self[mov.to].as_ref().map(|piece| (piece.score(), color));

        // update pawns
        let set: HashSet<_> = self
            .all_color_pieces(self[mov.from].as_ref().unwrap().color())
            .into_iter()
            .filter(|coord| {
                self[*coord]
                    .as_ref()
                    .is_some_and(|piece| piece.is_state(PawnState::JustDouble.into()))
            })
            .collect();
        for coord in set {
            self[coord]
                .as_mut()
                .unwrap()
                .set_state(PawnState::Already.into());
        }

        // move
        self.apply_move(mov, promoted);

        res
    }
    /// Without checking errors, move a piece.
    fn apply_move(&mut self, mov: &Movement, promoted: Option<Box<dyn Piece>>) {
        let piece = self[mov.from].as_mut().unwrap();
        if mov
            .special
            .as_ref()
            .is_some_and(|sm| sm == &SpecialMove::DoublePawn)
        {
            piece.set_state(PawnState::JustDouble.into());
        }

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
        let piece = self[mov.from].take().unwrap(); // .unwrap() to ensure it still exists.

        self[mov.to] = promoted.or(Some(piece));
        self[mov.to].as_mut().unwrap().set_pos(mov.to);
    }
    /// Returns the King's coordinates of the given color.
    fn find_king(&self, color: Color) -> Point {
        for (y, row) in self.iter().enumerate() {
            for (x, square) in row.iter().enumerate() {
                if square
                    .as_ref()
                    .is_some_and(|piece| piece.as_any().is::<King>() && piece.color() == color)
                {
                    return Point::new(x as isize, y as isize);
                }
            }
        }
        unreachable!("There should be a King");
    }
    /// `color` is the color of the king about to be captured
    pub fn check(&self, color: Color) -> Option<Movement> {
        let king_pos = self.find_king(color);
        self.all_moves(color.opposite())
            .into_iter()
            .find(|mov| mov.to == king_pos)
    }
    /// Can the player block the Check moving a piece? Returns the Movements that stops the check
    ///
    /// Returns an HashSet of all the moves with which the player can block the piece by
    /// eating it or putting a piece between it and the king.
    ///
    /// Doesn't check for the king moving itself (TODO might be added?)
    ///
    /// `color` is the color of the king about to be captured
    pub fn is_check_stoppable(&self, color: Color) -> HashSet<Movement> {
        let mut stop_cells = HashSet::new();
        let check_move = self.check(color).unwrap();
        // Adding to stop_cells
        stop_cells.insert(check_move.from);
        // Add more (bishop/rook/queen)
        if let Some(linearity) = check_move.linear() {
            let mut step = check_move.from + linearity;
            while step != check_move.to {
                stop_cells.insert(step);
                step += linearity;
            }
        }

        // Can be stopped?
        self.all_moves(color)
            .into_iter()
            .filter(|mov| {
                !self[mov.from].as_ref().unwrap().as_any().is::<King>()
                    && stop_cells.contains(&mov.to)
            })
            .collect()
    }
    /// `color` is the color of the king about to be captured
    pub fn checks_around(&self, color: Color) -> bool {
        self.filtered_move_set(self.find_king(color))
            .into_iter()
            .all(|mov| {
                let mut new_board = self.clone();
                new_board.do_move(&mov, None); // moving the king won't ever promote anything

                new_board.check(color).is_some()
            })
    }
    /// `color` is the color of the king about to be captured
    #[inline]
    pub fn checkmate(&self, color: Color) -> bool {
        // Return is check, all around check, and if check is not stoppable
        self.check(color).is_some()
            && self.checks_around(color)
            && self.is_check_stoppable(color).is_empty()
    }
    #[inline(always)]
    pub fn stalemate(&self, color: Color) -> bool {
        // No moves available and not check
        self.check(color).is_none() && self.all_moves(color).is_empty()
    }
}
