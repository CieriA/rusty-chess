use std::error::Error;

#[cfg(test)]
pub(crate) mod prelude;
mod game;
mod chessboard;
mod geomath;
mod pieces;

// TODO:
//  tie for lots of moves without moving pawns or eating pieces (50 or 75)
//  tie for repeated moves (3 or 5)
//  tie for not enough pieces (both players)


fn main() -> Result<(), Box<dyn Error>> {
    game::run()
}
