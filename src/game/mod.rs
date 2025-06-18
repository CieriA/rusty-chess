use std::{
    io::{self, Write},
    error::Error,
};
use std::collections::HashSet;
use crate::chessboard::Board;
use crate::pieces::types::Color;
use crate::geomath::Point;

const P1: &str = "White";
const P2: &str = "Black";

#[inline(always)]
fn p_name(color: Color) -> &'static str {
    if color.into() { P1 } else { P2 }
}

pub(super) fn run() -> Result<(), Box<dyn Error>> {
    print_instructions();

    let mut board = Board::default();
    let mut color = Color::default();

    loop {
        println!("It's {}'s turn", p_name(color));
        println!("{}", board);

        let mut from = String::new();
        print!("Piece coords: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut from)?;


        let mut to = String::new();
        print!("To: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut to)?;

        let from_cell = from.trim();
        let to_cell = to.trim();
        if from_cell.len() != 2 || to_cell.len() != 2 {
            println!("Invalid input.\n");
            continue;
        }
        let Ok(from) = Point::try_from(from_cell) else {
            println!("Invalid input.\n");
            continue;
        };
        let Ok(to) = Point::try_from(to_cell) else {
            println!("Invalid input.\n");
            continue;
        };

        let Some(piece) = board[from].as_ref() else {
            println!("Empty cell.");
            continue;
        };

        if piece.color() != color {
            println!("Not your piece.");
            continue;
        }
        /*if board[to].as_ref().is_some_and(|piece| piece.color() == color) {
            println!("Can't overlap two pieces.");
        }*/
        let Some(movement) = board
            .filtered_move_set(from)
            .into_iter()
            .find(|mov| mov.from == from && mov.to == to) else {
            println!("Invalid move.");
            continue;
        };

        // do move
        board.do_move(movement);
        color = color.opposite();

        // instant check for checkmate after the move
        if board.check(color).is_some() {
            if board.checkmate(color) {
                println!("{}", board);
                println!("{} lost.", p_name(color));
                break;
            }
        }
        if board.stalemate(color) {
            println!("{}", board);
            println!("It's a tie.");
            break;
        }
    }

    Ok(())
}

pub(crate) fn ask_upgrade() -> Result<char, Box<dyn Error>> {
    println!("Pawn's got to the last row.");

    let mut input = String::new();
    print!("Choose a piece to upgrade: (B/N/R/Q) ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)?;

    let chars = HashSet::from(["B", "N", "R", "Q"]);

    let input = input.trim().to_ascii_uppercase();
    if !chars.contains(&input.as_str()) {
        return Err("Invalid input.".into());
    }

    Ok(input.chars().nth(0).unwrap())
}

fn print_instructions() {
    println!("Chess!\n");
    println!("To play, write the coordinates of the piece you want to move and then the coordinates where you want it to go");
    println!("Example:\nPiece coords: E2\nTo: E4\n\n");
    println!("To promote a Pawn, write the first letter of the piece you want to promote (B/N/R/Q)")
}
