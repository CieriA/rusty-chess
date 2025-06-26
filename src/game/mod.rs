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

/// Engine of the game
#[derive(Default)]
pub(super) struct Game {
    /// Score of pieces eaten by white
    w_score: u8,
    /// Score of pieces eaten by black
    b_score: u8,
    /// Count for the 50 moves rule
    move_count: u8,
    /// Chessboard
    board: Board,
    /// Turn of the game (White / Black)
    turn: Color,
}

impl Game {
    /// How to play the game.
    #[inline]
    fn print_instructions() {
        println!("Chess!\n");
        println!("To play, write the coordinates of the piece you want to move and then the coordinates where you want it to go");
        println!("Example:\nPiece coords: E2\nTo: E4\n\n");
        println!("To promote a Pawn, write the first letter of the piece you want to promote (B/N/R/Q)")
    }
    /// Real score of white
    #[inline(always)]
    const fn white_score(&self) -> i8 {
        self.w_score as i8 - self.b_score as i8
    }
    /// Real score of black
    #[inline(always)]
    const fn black_score(&self) -> i8 {
        self.b_score as i8 - self.w_score as i8
    }
    /// A string of the scores to be printed
    fn score_str(&self) -> String {
        format!(
            "[{}: {}]",
            p_name(self.turn),
            self.get_printable_score(self.turn),
        )
    }
    #[inline]
    fn get_mut_score(&mut self, color: Color) -> &mut u8 {
        if color.into() {
            &mut self.w_score
        } else {
            &mut self.b_score
        }
    }
    #[inline]
    fn get_printable_score(&self, color: Color) -> i8 {
        if color.into() {
            self.white_score()
        } else {
            self.black_score()
        }
    }
    pub(super) fn run(&mut self) -> Result<(), Box<dyn Error>> {
        Self::print_instructions();

        loop {
            println!("It's {}'s turn", p_name(self.turn));
            println!("{}", self.score_str());
            println!("{}", self.board);

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

            let Some(piece) = self.board[from].as_ref() else {
                println!("Empty cell.");
                continue;
            };

            if piece.color() != self.turn {
                println!("Not your piece.");
                continue;
            }
            let Some(movement) = self.board
                .filtered_move_set(from)
                .into_iter()
                .find(|mov| mov.from == from && mov.to == to) else {
                println!("Invalid move.");
                continue;
            };

            { // control if the move would lead to a check
                let mut board = self.board.clone();
                let mut score = self.get_printable_score(self.turn.opposite()); // score clone
                if let Some((new_score, ..)) = board.do_move(movement.clone()) {
                    score -= new_score as i8; // this will be seen by the losing player
                }

                if board.check(self.turn).is_some() {
                    println!("Invalid move.");
                    continue;
                }

                if board.checkmate(self.turn.opposite()) {
                    println!("[{}: {}]", p_name(self.turn.opposite()), score);
                    println!("{}", board);
                    println!("{} lost.", p_name(self.turn.opposite()));
                    break;
                }
                if board.stalemate(self.turn.opposite()) {
                    println!("{}", board);
                    println!("It's a tie.");
                    break;
                }
            }
            // Only kings on the board
            if self.board.all_pieces().len() == 2 {
                println!("{}", self.score_str());
                println!("{}", self.board);
                println!("It's a tie.");
                break;
            }

            // 50moves rule's count
            if self.board[movement.from].as_ref().unwrap().is_pawn() || self.board[movement.to].is_some() {
                self.move_count = 0;
            } else {
                self.move_count += 1;
            }
            // Tie by 50 moves rule
            if self.move_count == 50 {
                println!("{}", self.score_str());
                println!("{}", self.board);
                println!("It's a tie.");
                break;
            }

            if let Some((score, color)) = self.board.do_move(movement) {
                *self.get_mut_score(color) += score;
            }
            
            self.turn = self.turn.opposite();
        }

        Ok(())
    }
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

    Ok(input.chars().next().unwrap())
}
