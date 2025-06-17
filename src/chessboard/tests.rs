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
                None,
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
                None,
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
    king.set_state(PieceState::Already);
    board[king_pos] = Some(Box::new(king.clone()));
    
    let rooks = [(0, 0), (7, 0)].map(Point::from);
    for rook in rooks {
        board[rook] = Some(Box::new(Rook::new(color, rook)));
    }
    
    assert_eq!(
        board.filtered_move_set(king_pos),
        IndexSet::from([
            Movement::new(king_pos, Point::new(3, 0), None, Some(Direction::Left)),
            Movement::new(king_pos, Point::new(5, 0), None, Some(Direction::Right)),
            Movement::new(king_pos, Point::new(4, 1), None, Some(Direction::Up)),
            Movement::new(king_pos, Point::new(3, 1), None, Some(Direction::UpLeft)),
            Movement::new(king_pos, Point::new(5, 1), None, Some(Direction::UpRight)),
        ])
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
        assert!(board.do_move(mov));
    }
}
