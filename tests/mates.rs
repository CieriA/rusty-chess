mod moves;

use {rusty_chess::prelude::*, std::error::Error};

#[test]
fn scholar_mate() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("E2", "E4"),
        ("E7", "E5"),
        ("D1", "H5"),
        ("B8", "C6"),
        ("F1", "C4"),
        ("G8", "F6"),
        ("H5", "F7"), // checkmate
    ];

    // game simulation
    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }

    assert!(game.board.checkmate(Color::Black));

    Ok(())
}

#[test]
fn fools_mate() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("F2", "F3"),
        ("E7", "E6"),
        ("G2", "G4"),
        ("D8", "H4"), // checkmate
    ];

    // game simulation
    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }

    assert!(game.board.checkmate(Color::White));

    Ok(())
}
