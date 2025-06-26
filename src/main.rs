use std::error::Error;
use crate::game::Game;

#[cfg(test)]
pub(crate) mod prelude;
mod game;
mod chessboard;
mod geomath;
mod pieces;

fn main() -> Result<(), Box<dyn Error>> {
    Game::default().run()
}
