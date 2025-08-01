use {rusty_chess::prelude::*, std::error::Error};

pub fn do_move(
    game: &mut Game,
    from: &str,
    to: &str,
    promoted: Option<Box<dyn Piece>>,
) -> Result<Movement, Box<dyn Error>> {
    let from = Point::try_from(from)?;
    let to = Point::try_from(to)?;
    let mov = game
        .board
        .filtered_move_set(from)
        .into_iter()
        .find(|mov| mov.from == from && mov.to == to)
        .ok_or(format!("Move not valid: from: {from} to: {to}"))?;

    game.fifty_moves(&mov);
    game.board.do_move(&mov, promoted);

    Ok(mov)
}
