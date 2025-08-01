mod moves;

use {rusty_chess::prelude::*, std::error::Error};

#[test]
fn fifty_moves() -> Result<(), Box<dyn Error>> {
    let mut game = Game::default();
    let movements = [("B1", "C3"), ("G8", "F6"), ("C3", "B1"), ("F6", "G8")]
        .into_iter()
        .cycle();

    for (i, (from, to)) in movements.take(50).enumerate() {
        moves::do_move(&mut game, from, to, None)?;
        assert_eq!(game.move_count, i as u8 + 1);
    }

    moves::do_move(&mut game, "E2", "E4", None)?;
    assert_eq!(game.move_count, 0);

    Ok(())
}
