use indexmap::IndexSet;
use crate::chessboard::Board;
use crate::geomath::Point;
use crate::pieces::{rook::Rook, pawn::Pawn, bishop::Bishop, types::{Color, Piece, PawnState}};

#[cfg(test)]
pub(crate) mod prelude;
mod game;
mod chessboard;
mod geomath;
mod pieces;

fn main() {
    let mut board = Board::empty();
    let color = Color::Black;
    let pos = Point::new(2, 3);
    let piece = Pawn::new(color, pos);
    board[pos] = Some(Box::new(piece.clone()));

    let mut pieces: Vec<Box<dyn Piece>> = vec![
        Box::new(Rook::new(color.opposite(), Point::new(1, 2))), // Eatable
        Box::new(Pawn::new(color.opposite(), Point::new(3, 3))), // En Passant
        Box::new(Bishop::new(color, Point::new(2, 2))), // Cannot move
    ];
    pieces[1].set_state(PawnState::JustDouble.into());

    for piece in pieces {
        let pos = piece.pos();
        board[pos] = Some(piece);
    }
    
    println!("{}", board);
}
