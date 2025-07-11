use crate::game::Game;
use std::error::Error;

mod chessboard;
mod game;
mod geomath;
mod pieces;
#[cfg(test)]
pub(crate) mod prelude;

fn main() -> Result<(), Box<dyn Error>> {
    Game::default().run()
}
