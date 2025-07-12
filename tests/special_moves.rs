mod setup;

use rusty_chess::prelude::*;
use std::error::Error;

#[test]
fn promote() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let moves = vec![
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
    for (from, to) in moves {
        setup::do_move(&mut game, from, to, None)?;
    }
    // promoting move
    setup::do_move(
        &mut game,
        "E7",
        "E8",
        Some(Box::new(Rook::new(Color::White, Point::new(4, 7)))),
    )?;

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
    let moves = vec![
        ("G2", "G3"),
        ("E7", "E5"),
        ("F1", "H3"),
        ("E5", "E4"),
        ("G1", "F3"),
        ("D7", "D5"),
        ("E1", "G1"), // castle
    ];

    // game simulation
    for (from, to) in moves {
        setup::do_move(&mut game, from, to, None)?;
    }

    assert!(
        game.board[Point::new(6, 0)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<King>()
    );
    assert!(
        game.board[Point::new(5, 0)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<Rook>()
    );
    assert!(game.board[Point::new(7, 0)].is_none());
    assert!(game.board[Point::new(4, 0)].is_none());

    Ok(())
}

#[test]
fn long_castle() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let moves = vec![
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

    for (from, to) in moves {
        setup::do_move(&mut game, from, to, None)?;
    }

    assert!(
        game.board[Point::new(3, 0)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<Rook>()
    );
    assert!(
        game.board[Point::new(2, 0)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<King>()
    );
    assert!(game.board[Point::new(0, 0)].is_none());
    assert!(game.board[Point::new(1, 0)].is_none());
    assert!(game.board[Point::new(4, 0)].is_none());

    Ok(())
}

#[test]
fn en_passant() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let moves = vec![
        ("E2", "E4"),
        ("B8", "A6"),
        ("E4", "E5"),
        ("D7", "D5"),
        ("E5", "D6"), // en passant
    ];

    for (from, to) in moves {
        setup::do_move(&mut game, from, to, None)?;
    }

    assert!(
        game.board[Point::new(3, 5)]
            .as_ref()
            .unwrap()
            .as_any()
            .is::<Pawn>()
    );
    assert!(game.board[Point::new(3, 4)].is_none());

    Ok(())
}
