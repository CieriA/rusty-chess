use rusty_chess::game::Game;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Game::default().run()
}
