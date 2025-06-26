use crate::prelude::*;
use indexmap::IndexSet;
// `.filtered_move_set()` tests

// Pawns
#[test]
fn pawn_alone() {
    let mut board = Board::empty();
    let pos = Point::new(3, 1);
    let piece = Pawn::new(Color::White, pos);
    board[pos] = Some(Box::new(piece.clone()));
    assert_eq!(
        board.filtered_move_set(pos),
        [
            piece.to_movement(
                Point::new(0, 1),
                Some(SpecialMove::CannotEat),
                Some(Direction::Up),
            ),
            piece.to_movement(
                Point::new(0, 2),
                Some(SpecialMove::DoublePawn),
                Some(Direction::Up),
            ),
        ]
            .into_iter()
            .flatten()
            .collect::<IndexSet<_>>()
    );
}
#[test]
fn pawn_moved() {
    let mut board = Board::empty();
    let color = Color::Black;
    let pos = Point::new(6, 4);
    let mut piece = Pawn::new(color, pos);
    piece.set_state(PawnState::Already.into());
    board[pos] = Some(Box::new(piece.clone()));
    
    assert_eq!(
        board.filtered_move_set(pos),
        [
            piece.to_movement(
                Point::new(0, 1),
                Some(SpecialMove::CannotEat),
                Some(Direction::Up),
            ),
        ]
            .into_iter()
            .flatten()
            .collect::<IndexSet<_>>()
    );
}
#[test]
fn pawn_eat_only() {
    let mut board = Board::empty();
    let color = Color::White;
    let pos = Point::new(3, 1);
    let piece = Pawn::new(color, pos);
    board[pos] = Some(Box::new(piece.clone()));

    let pieces: Vec<Box<dyn Piece>> = vec![
        Box::new(Bishop::new(color.opposite(), Point::new(2, 2))), // Eatable
        Box::new(Pawn::new(color.opposite(), Point::new(4, 2))), // Eatable
        Box::new(Knight::new(color, Point::new(3, 2))), // Cannot move
    ];

    for piece in pieces {
        let pos = piece.pos();
        board[pos] = Some(piece);
    }

    assert_eq!(
        board.filtered_move_set(pos),
        [
            piece.to_movement(
                Point::new(1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpRight),
            ),
            piece.to_movement(
                Point::new(-1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpLeft),
            ),
        ]
            .into_iter()
            .flatten()
            .collect::<IndexSet<_>>()
    );
}
#[test]
fn en_passant_and_eat() {
    let mut board = Board::empty();
    let color = Color::Black;
    let pos = Point::new(2, 3);
    let piece = Pawn::new(color, pos);
    board[pos] = Some(Box::new(piece.clone()));
    
    let mut en_passant_piece = Pawn::new(color.opposite(), Point::new(3, 3));
    en_passant_piece.set_state(PawnState::JustDouble.into());
    board[en_passant_piece.pos()] = Some(Box::new(en_passant_piece.clone()));
    
    board[Point::new(1, 2)] = Some(Box::new(Rook::new(color.opposite(), Point::new(1, 2))));
    board[Point::new(2, 2)] = Some(Box::new(Bishop::new(color, Point::new(2, 2))));

    assert_eq!(
        board.filtered_move_set(pos),
        [
            piece.to_movement(
                Point::new(-1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpLeft), // En Passant
            ),
            piece.to_movement(
                Point::new(1, 1),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpRight),
            ),
        ]
            .into_iter()
            .flatten()
            .collect::<IndexSet<_>>()
    );
}
#[test]
fn pawn_stuck() {
    let mut board = Board::empty();
    let mut piece = Pawn::new(Color::White, Point::default());
    piece.set_state(PawnState::Already.into());
    board[Point::new(0, 1)] = Some(Box::new(piece.clone()));
    board[Point::default()] = Some(Box::new(piece));
    assert!(board.filtered_move_set(Point::default()).is_empty());
}
#[test]
fn cannot_eat_straight() {
    let mut board = Board::empty();
    let pawn_pos = Point::new(3, 1);
    let bishop_pos = Point::new(3, 2);
    let color = Color::White;
    
    board[pawn_pos] = Some(Box::new(Pawn::new(color, pawn_pos)));
    board[bishop_pos] = Some(Box::new(Bishop::new(color.opposite(), bishop_pos)));
    
    let movements = [
        Movement::new(pawn_pos, Point::new(3, 2), None, Some(Direction::Up)),
        Movement::new(pawn_pos, Point::new(3, 3), Some(SpecialMove::DoublePawn), Some(Direction::Up)),
    ];
    for mov in movements {
        assert!(
            !board.filtered_move_set(pawn_pos).contains(&mov)
        );
    }
}

// Knights
#[test]
fn knight_stuck() {
    let mut board = Board::empty();
    let color = Color::Black;
    let piece = Knight::new(color, Point::default());
    board[Point::default()] = Some(Box::new(piece));

    board[Point::new(1, 2)] = Some(Box::new(Rook::new(color, Point::new(1, 2))));
    board[Point::new(2, 1)] = Some(Box::new(Rook::new(color, Point::new(2, 1))));
    assert!(board.filtered_move_set(Point::default()).is_empty());
}

// Bishops
#[test]
fn bishop_colliding() {
    let mut board = Board::empty();
    let pos = Point::new(3, 4);
    let color = Color::White;
    let piece = Bishop::new(color, pos);
    board[pos] = Some(Box::new(piece.clone()));

    let pieces: Vec<Box<dyn Piece>> = vec![
        Box::new(Queen::new(color.opposite(), Point::new(2, 3))), // Next to but eatable
        Box::new(Pawn::new(color, Point::new(5, 2))), // Colliding, not eatable
        Box::new(Pawn::new(color, Point::new(6, 7))), // Last square, not eatable
    ];

    for piece in pieces {
        let pos = piece.pos();
        board[pos] = Some(piece);
    }

    assert_eq!(
        board.filtered_move_set(pos),
        [
            piece.to_movement(
                Point::new(1, 1),
                None,
                Some(Direction::UpRight),
            ),
            piece.to_movement(
                Point::new(2, 2),
                None,
                Some(Direction::UpRight),
            ),

            piece.to_movement(
                Point::new(-1, -1),
                None,
                Some(Direction::DownLeft),
            ),

            piece.to_movement(
                Point::new(1, -1),
                None,
                Some(Direction::DownRight),
            ),

            piece.to_movement(
                Point::new(-1, 1),
                None,
                Some(Direction::UpLeft),
            ),
            piece.to_movement(
                Point::new(-2, 2),
                None,
                Some(Direction::UpLeft),
            ),
            piece.to_movement(
                Point::new(-3, 3),
                None,
                Some(Direction::UpLeft),
            ),
        ]
            .into_iter()
            .flatten()
            .collect::<IndexSet<_>>()
    )
}
#[test]
fn bishop_stuck() {
    let board = Board::default();
    assert_eq!(
        board.filtered_move_set(Point::new(2, 0)),
        IndexSet::new()
    );
    assert_eq!(
        board.filtered_move_set(Point::new(2, 7)),
        IndexSet::new()
    );
    assert_eq!(
        board.filtered_move_set(Point::new(5, 0)),
        IndexSet::new()
    );
    assert!(board.filtered_move_set(Point::new(5, 7)).is_empty(),);
}

// Rooks
#[test]
fn rook_not_colliding() {
    let mut board = Board::empty();
    let pos = Point::new(7, 2);
    let color = Color::Black;
    let piece = Rook::new(color, pos);
    board[pos] = Some(Box::new(piece.clone()));
    
    // other pieces non-in trail with the rook
    let pieces: Vec<Box<dyn Piece>> = vec![
        Box::new(Queen::new(Color::White, Point::new(0, 3))),
        Box::new(King::new(Color::Black, Point::new(3, 5))),
        Box::new(Knight::new(Color::Black, Point::new(4, 6))),
        Box::new(Pawn::new(Color::White, Point::new(6, 1))),
    ];
    
    for piece in pieces {
        let pos = piece.pos();
        board[pos] = Some(piece);
    }
    
    assert_eq!(
        board.filtered_move_set(pos),
        piece.move_set()
    );
}
#[test]
fn rook_stuck() {
    let board = Board::default();
    assert_eq!(
        board.filtered_move_set(Point::new(0, 0)),
        IndexSet::new()
    );
    assert_eq!(
        board.filtered_move_set(Point::new(0, 7)),
        IndexSet::new()
    );
    assert_eq!(
        board.filtered_move_set(Point::new(7, 0)),
        IndexSet::new()
    );
    assert!(board.filtered_move_set(Point::new(7, 7)).is_empty());
}

// Queens
#[test]
fn queen_eat() { 
    // Queen has pieces all around.
    // Assert the pos is the same as with a King with no other pieces.
    
    let mut board = Board::empty();
    let mut board2 = Board::empty();
    let color = Color::White;
    let pos = Point::new(4, 3);
    
    let queen = Queen::new(color, pos);
    let king = King::new(color, pos);
    
    board[pos] = Some(Box::new(queen.clone()));
    board2[pos] = Some(Box::new(king.clone()));
    
    // Blocking queen
    Point::all_around(1)
        .into_iter()
        .map(|(point, dir)| (point + pos, dir))
        .map(|(pos, _)| Knight::new(color.opposite(), pos))
        .for_each(|knight| board[knight.pos()] = Some(Box::new(knight.clone())));
    
    assert_eq!(
        board.filtered_move_set(pos),
        board2.filtered_move_set(pos),
    )
}
#[test]
fn queen_stuck() {
    let board = Board::default();
    assert_eq!(
        board.filtered_move_set(Point::new(3, 0)),
        IndexSet::new()
    );
    assert!(board.filtered_move_set(Point::new(3, 7)).is_empty(),);
}

// Kings
#[test]
fn king_stuck() {
    let board = Board::default();
    assert_eq!(
        board.filtered_move_set(Point::new(4, 0)),
        IndexSet::new()
    );
    assert!(board.filtered_move_set(Point::new(4, 7)).is_empty(),);
}
#[test]
fn king_no_castle() {
    let mut board = Board::empty();
    let king_pos = Point::new(4, 0);
    let color = Color::White;
    let mut king = King::new(color, king_pos);
    king.set_state(PieceState::Already.into());
    board[king_pos] = Some(Box::new(king.clone()));
    
    let rooks = [(0, 0), (7, 0)].map(Point::from);
    for rook in rooks {
        board[rook] = Some(Box::new(Rook::new(color, rook)));
    }
    
    assert!(
        board
            .filtered_move_set(king_pos)
            .into_iter()
            .all(|mov| mov.special.is_none())
    )
}
#[test]
fn moved_rooks() {
    let mut board = Board::empty();
    let king_pos = Point::new(4, 0);
    let color = Color::White;
    let king = King::new(color, king_pos);
    board[king_pos] = Some(Box::new(king.clone()));

    let rooks = [(0, 0), (7, 0)].map(Point::from);
    for rook_pos in rooks {
        let mut rook = Rook::new(color, rook_pos);
        rook.set_state(PieceState::Already.into());
        board[rook_pos] = Some(Box::new(rook));
    }

    assert!(
        board
            .filtered_move_set(king_pos)
            .into_iter()
            .all(|mov| mov.special.is_none())
    )
}
#[test]
fn castle_blocked() {
    let mut board = Board::empty();
    let king_pos = Point::new(4, 0);
    let color = Color::White;
    let king = King::new(color, king_pos);
    board[king_pos] = Some(Box::new(king.clone()));

    let rooks = [(0, 0), (7, 0)].map(Point::from);
    for rook in rooks {
        board[rook] = Some(Box::new(Rook::new(color, rook)));
    }
    let bishop_pos = Point::new(3, 2); // blocking short castles
    board[bishop_pos] = Some(Box::new(Bishop::new(color.opposite(), bishop_pos)));

    let rook_pos = Point::new(2, 7); // blocking long castle
    board[rook_pos] = Some(Box::new(Rook::new(color.opposite(), rook_pos)));

    assert!(
        board
            .filtered_move_set(king_pos)
            .into_iter()
            .all(|mov| mov.special.is_none())
    )
}

// `.do_move()` tests
#[test]
fn knights_moves() {
    let mut board = Board::default();
    
    let movements = [
        Movement::new(Point::new(1, 0), Point::new(2, 2), None, None),
        Movement::new(Point::new(6, 0), Point::new(5, 2), None, None),
        Movement::new(Point::new(5, 2), Point::new(3, 3), None, None),
        Movement::new(Point::new(1, 7), Point::new(0, 5), None, None),
    ];
    
    for mov in movements {
        assert!(board.do_move(mov).is_ok());
    }
}

// `.check()` tests
#[test]
fn rook_check() {
    let mut board = Board::empty();
    let color = Color::White;
    let king = King::new(color, Point::new(3, 7));
    let rook = Rook::new(color.opposite(), Point::new(5, 7));

    board[Point::new(3, 7)] = Some(Box::new(king));
    board[Point::new(5, 7)] = Some(Box::new(rook));
    
    assert!(board.check(color).is_some());
}

#[test]
fn bishop_check() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king = King::new(color, Point::new(1, 0));
    let bishop = Bishop::new(color.opposite(), Point::new(7, 6));

    board[Point::new(1, 0)] = Some(Box::new(king));
    board[Point::new(7, 6)] = Some(Box::new(bishop));

    assert!(board.check(color).is_some());
}

#[test]
fn knight_check() {
    let mut board = Board::empty();
    let color = Color::White;
    let king = King::new(color, Point::new(4, 5));
    let knight = Knight::new(color.opposite(), Point::new(6, 4));

    board[Point::new(4, 5)] = Some(Box::new(king));
    board[Point::new(6, 4)] = Some(Box::new(knight));

    assert!(board.check(color).is_some());
}

#[test]
fn queen_check() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king = King::new(color, Point::new(0, 0));
    let queen = Queen::new(color.opposite(), Point::new(7, 7));

    board[Point::new(0, 0)] = Some(Box::new(king));
    board[Point::new(7, 7)] = Some(Box::new(queen));

    assert!(board.check(color).is_some());
}

#[test]
fn pawn_check() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king = King::new(color, Point::new(2, 2));
    let pawn = Pawn::new(color.opposite(), Point::new(3, 1));
    
    board[Point::new(2, 2)] = Some(Box::new(king));
    board[Point::new(3, 1)] = Some(Box::new(pawn));
    
    assert!(board.check(color).is_some());
}

// `.is_check_stoppable()` tests
#[test]
fn rook_stoppable() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king_pos = Point::new(0, 0);
    let rook_pos = Point::new(7, 0);
    let pawn_eat = Point::new(6, 1);
    
    let king = King::new(color, king_pos);
    let rook = Rook::new(color.opposite(), rook_pos);
    let pawn = Pawn::new(color, pawn_eat);
    
    board[king_pos] = Some(Box::new(king));
    board[rook_pos] = Some(Box::new(rook));
    board[pawn_eat] = Some(Box::new(pawn));
    
    assert!(!board.is_check_stoppable(color).is_empty());
}

#[test]
fn bishop_stoppable() {
    let mut board = Board::empty();
    let color = Color::White;
    let king_pos = Point::new(7, 7);
    let bishop_pos = Point::new(0, 0);
    let knight_eat = Point::new(2, 1);
    
    let king = King::new(color, king_pos);
    let bishop = Bishop::new(color.opposite(), bishop_pos);
    let knight = Knight::new(color, knight_eat);
    
    board[king_pos] = Some(Box::new(king));
    board[bishop_pos] = Some(Box::new(bishop));
    board[knight_eat] = Some(Box::new(knight));
    
    assert!(!board.is_check_stoppable(color).is_empty());
}

#[test]
fn queen_stoppable() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king_pos = Point::new(7, 0);
    let queen_pos = Point::new(0, 7);
    let rook_eat = Point::new(0, 1);
    
    let king = King::new(color, king_pos);
    let queen = Queen::new(color.opposite(), queen_pos);
    let rook = Rook::new(color, rook_eat);
    
    board[king_pos] = Some(Box::new(king));
    board[queen_pos] = Some(Box::new(queen));
    board[rook_eat] = Some(Box::new(rook));
    
    assert!(!board.is_check_stoppable(color).is_empty());
}
#[test]
fn not_stoppable() {
    let mut board = Board::empty();
    let color = Color::White;
    let king_pos = Point::new(7, 0);
    let queen_pos = Point::new(0, 7);
    let pawn_pos = Point::new(3, 1);

    let king = King::new(color, king_pos);
    let queen = Queen::new(color.opposite(), queen_pos);
    let pawn = Pawn::new(color, pawn_pos);

    board[king_pos] = Some(Box::new(king));
    board[queen_pos] = Some(Box::new(queen));
    board[pawn_pos] = Some(Box::new(pawn));
    
    assert!(board.is_check_stoppable(color).is_empty());
}

// `.checkmate()` tests
#[test]
fn rook_mate() {
    let mut board = Board::empty();
    let color = Color::White;
    let first_pos = Point::new(6, 5);
    let second_pos = Point::new(7, 7);
    let rook_pos = Point::new(5, 7);
    
    let first = King::new(color.opposite(), first_pos);
    let second = King::new(color, second_pos);
    let rook = Rook::new(color.opposite(), rook_pos);

    board[first_pos] = Some(Box::new(first));
    board[second_pos] = Some(Box::new(second));
    board[rook_pos] = Some(Box::new(rook));
    
    assert!(board.checkmate(color));
}

#[test]
fn bishops_mate() {
    let mut board = Board::empty();
    let color = Color::Black;
    let first_pos = Point::new(1, 5);
    let second_pos = Point::new(0, 7);
    let bishop_pos = Point::new(2, 5);
    let bishop_pos2 = Point::new(2, 6);

    let first = King::new(color.opposite(), first_pos);
    let second = King::new(color, second_pos);
    let bishop = Bishop::new(color.opposite(), bishop_pos);
    let bishop2 = Bishop::new(color.opposite(), bishop_pos2);

    board[first_pos] = Some(Box::new(first));
    board[second_pos] = Some(Box::new(second));
    board[bishop_pos] = Some(Box::new(bishop));
    board[bishop_pos2] = Some(Box::new(bishop2));

    assert!(board.checkmate(color));
}

#[test]
fn queen_mate() {
    let mut board = Board::empty();
    let color = Color::White;
    let first_pos = Point::new(4, 5);
    let second_pos = Point::new(5, 7);
    let queen_pos = Point::new(5, 6);

    let first = King::new(color.opposite(), first_pos);
    let second = King::new(color, second_pos);
    let queen = Queen::new(color.opposite(), queen_pos);

    board[first_pos] = Some(Box::new(first));
    board[second_pos] = Some(Box::new(second));
    board[queen_pos] = Some(Box::new(queen));

    println!("{}", board);
    assert!(board.checkmate(color));
}

#[test]
fn bishop_knight_mate() {
    let mut board = Board::empty();
    let color = Color::Black;
    let first_pos = Point::new(6, 5);
    let second_pos = Point::new(7, 7);
    let bishop_pos = Point::new(5, 5);
    let knight_pos = Point::new(7, 5);

    let first = King::new(color.opposite(), first_pos);
    let second = King::new(color, second_pos);
    let bishop = Bishop::new(color.opposite(), bishop_pos);
    let knight = Knight::new(color.opposite(), knight_pos);

    board[first_pos] = Some(Box::new(first));
    board[second_pos] = Some(Box::new(second));
    board[bishop_pos] = Some(Box::new(bishop));
    board[knight_pos] = Some(Box::new(knight));

    assert!(board.checkmate(color));
}

#[test]
fn two_rooks() {
    let mut board = Board::empty();
    let color = Color::Black;
    let king_pos = Point::new(7, 7);
    let rook_pos = Point::new(7, 0);
    let rook_pos2 = Point::new(6, 0);

    let king = King::new(color, king_pos);
    let rook = Rook::new(color.opposite(), rook_pos);
    let rook2 = Rook::new(color.opposite(), rook_pos2);

    board[king_pos] = Some(Box::new(king));
    board[rook_pos] = Some(Box::new(rook));
    board[rook_pos2] = Some(Box::new(rook2));

    assert!(board.checkmate(color));
}

#[test]
fn base() { // .checkmate() doesn't work if the board is empty
    let board = Board::default();
    assert!(!board.checkmate(Color::White));
    assert!(!board.checkmate(Color::Black));
    let mut board = Board::empty();
    board[Point::default()] = Some(Box::new(King::new(Color::White, Point::default())));
    board[Point::new(7, 2)] = Some(Box::new(King::new(Color::Black, Point::new(7, 2))));
    assert!(!board.checkmate(Color::White));
    assert!(!board.checkmate(Color::Black));
}
