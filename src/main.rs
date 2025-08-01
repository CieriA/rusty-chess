use {rusty_chess::game::Game, std::error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    Game::default().run()
}
