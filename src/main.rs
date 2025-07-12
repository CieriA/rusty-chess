use crate::game::Game;
use std::error::Error;

mod chessboard;
mod game;
mod geomath;
#[cfg(test)]
pub(crate) mod prelude;
mod types;

fn main() -> Result<(), Box<dyn Error>> {
    Game::default().run()
}
