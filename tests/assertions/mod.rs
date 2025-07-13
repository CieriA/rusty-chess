use rusty_chess::prelude::*;

#[inline]
pub fn assert_presence<P: Piece>(game: &Game, coord: Point) {
    assert!(game.board[coord].as_ref().unwrap().as_any().is::<P>());
}

#[inline]
pub fn assert_empty(game: &Game, coord: Point) {
    assert!(game.board[coord].is_none());
}
