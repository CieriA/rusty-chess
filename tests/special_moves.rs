mod assertions;
mod moves;

use {
    assertions::{assert_empty, assert_presence},
    rusty_chess::prelude::*,
    std::error::Error,
};

#[test]
fn promote() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("E2", "E4"),
        ("E7", "E5"),
        ("D2", "D4"),
        ("E5", "D4"),
        ("E4", "E5"),
        ("E8", "E7"),
        ("E5", "E6"),
        ("E7", "D6"),
        ("E6", "E7"),
        ("D6", "D5"),
    ];

    // game simulation
    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }
    // promoting move
    moves::do_move(
        &mut game,
        "E7",
        "E8",
        Some(Box::new(Rook::new(Color::White, Point::new(4, 7)))),
    )?;

    assert_presence::<Rook>(&game, Point::new(4, 7));
    assert!(
        game.board[Point::new(4, 7)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<Rook>()
    );

    Ok(())
}

#[test]
fn short_castle() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("G2", "G3"),
        ("E7", "E5"),
        ("F1", "H3"),
        ("E5", "E4"),
        ("G1", "F3"),
        ("D7", "D5"),
        ("E1", "G1"), // castle
    ];

    // game simulation
    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }

    assert_presence::<King>(&game, Point::new(6, 0));
    assert_presence::<Rook>(&game, Point::new(5, 0));
    assert_empty(&game, Point::new(7, 0));
    assert_empty(&game, Point::new(4, 0));

    Ok(())
}

#[test]
fn long_castle() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("D2", "D4"),
        ("E7", "E6"),
        ("D1", "D3"),
        ("D7", "D6"),
        ("C1", "E3"),
        ("E6", "E5"),
        ("B1", "A3"),
        ("D6", "D5"),
        ("E1", "C1"), // castle
    ];

    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }

    assert_presence::<Rook>(&game, Point::new(3, 0));
    assert_presence::<King>(&game, Point::new(2, 0));
    assert_empty(&game, Point::new(0, 0));
    assert_empty(&game, Point::new(1, 0));
    assert_empty(&game, Point::new(4, 0));

    Ok(())
}

#[test]
fn en_passant() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [
        ("E2", "E4"),
        ("B8", "A6"),
        ("E4", "E5"),
        ("D7", "D5"),
        ("E5", "D6"), // en passant
    ];

    for (from, to) in movements {
        moves::do_move(&mut game, from, to, None)?;
    }

    assert_presence::<Pawn>(&game, Point::new(3, 5));
    assert_empty(&game, Point::new(3, 4));

    Ok(())
}
