mod setup;

use rusty_chess::prelude::*;
use std::error::Error;

#[test]
fn scholar_mate() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let moves = vec![
        ("E2", "E4"),
        ("E7", "E5"),
        ("D1", "H5"),
        ("F8", "C5"),
        ("F1", "C4"),
        ("G8", "F6"),
        ("H5", "F7"),
    ];

    // game simulation
    for (from, to) in moves {
        setup::do_move(&mut game, from, to, None)?;
    }

    assert!(game.board.checkmate(Color::Black));

    Ok(())
}
