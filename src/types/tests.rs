use crate::prelude::*;
use indexmap::IndexSet;

#[test]
fn pawn() {
    let pawn = Pawn::new(PieceColor::Black, Point::try_from("e7").unwrap(), CELL_SIZE);
    assert_eq!(
        pawn.move_set(),
        IndexSet::from([
            Movement::new(
                Point::new(4, 6),
                Point::new(4, 5),
                Some(SpecialMove::CannotEat),
                Some(Direction::Up)
            ),
            Movement::new(
                Point::new(4, 6),
                Point::new(4, 4),
                Some(SpecialMove::DoublePawn),
                Some(Direction::Up)
            ),
            Movement::new(
                Point::new(4, 6),
                Point::new(3, 5),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpRight)
            ),
            Movement::new(
                Point::new(4, 6),
                Point::new(5, 5),
                Some(SpecialMove::PawnEat),
                Some(Direction::UpLeft)
            )
        ])
    );
}

#[test]
fn bishop() {
    let bishop = Bishop::new(PieceColor::White, Point::try_from("b2").unwrap(), CELL_SIZE);
    let start = Point::new(1, 1);
    assert_eq!(
        bishop.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(2, 2), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(3, 3), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(4, 4), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(5, 5), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(6, 6), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(7, 7), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(0, 2), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(0, 0), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(2, 0), None, Some(Direction::DownRight)),
        ])
    )
}
#[test]
fn bishop_reverse() {
    let bishop = Bishop::new(PieceColor::Black, Point::try_from("g6").unwrap(), CELL_SIZE);
    let start = Point::new(6, 5);
    assert_eq!(
        bishop.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(5, 4), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(4, 3), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(3, 2), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(2, 1), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(1, 0), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(4, 7), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(5, 6), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(7, 6), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(7, 4), None, Some(Direction::DownRight)),
        ])
    );
}

#[test]
fn rook() {
    let rook = Rook::new(PieceColor::Black, Point::try_from("h8").unwrap(), CELL_SIZE);
    let start = Point::new(7, 7);
    assert_eq!(
        rook.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(6, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(5, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(4, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(3, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(2, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(1, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(0, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(7, 6), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 5), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 4), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 3), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 2), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 1), None, Some(Direction::Down)),
            Movement::new(start, Point::new(7, 0), None, Some(Direction::Down)),
        ])
    )
}

#[test]
fn knight() {
    let knight = Knight::new(PieceColor::White, Point::try_from("d4").unwrap(), CELL_SIZE);
    let start = Point::new(3, 3);
    assert_eq!(
        knight.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(2, 5), None, None),
            Movement::new(start, Point::new(4, 5), None, None),
            Movement::new(start, Point::new(5, 4), None, None),
            Movement::new(start, Point::new(5, 2), None, None),
            Movement::new(start, Point::new(4, 1), None, None),
            Movement::new(start, Point::new(2, 1), None, None),
            Movement::new(start, Point::new(1, 2), None, None),
            Movement::new(start, Point::new(1, 4), None, None),
        ])
    )
}
#[test]
fn starting_knight() {
    let knight = Knight::new(PieceColor::White, Point::try_from("b1").unwrap(), CELL_SIZE);
    let start = Point::new(1, 0);
    assert_eq!(
        knight.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(0, 2), None, None),
            Movement::new(start, Point::new(2, 2), None, None),
            Movement::new(start, Point::new(3, 1), None, None),
        ])
    )
}

#[test]
fn starting_king() {
    let king = King::new(PieceColor::Black, Point::try_from("e8").unwrap(), CELL_SIZE);
    let start = Point::new(4, 7);
    assert_eq!(
        king.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(3, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(5, 7), None, Some(Direction::Right)),
            Movement::new(start, Point::new(4, 6), None, Some(Direction::Down)),
            Movement::new(start, Point::new(3, 6), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(5, 6), None, Some(Direction::DownRight)),
            Movement::new(
                start,
                Point::new(2, 7),
                Some(SpecialMove::LongCastle),
                Some(Direction::Left)
            ),
            Movement::new(
                start,
                Point::new(6, 7),
                Some(SpecialMove::ShortCastle),
                Some(Direction::Right)
            ),
        ])
    )
}
#[test]
fn king_around() {
    let start = Point::new(5, 4);
    let moves = Point::all_around(1)
        .into_iter()
        .map(|(p, _)| p + start)
        .collect::<IndexSet<_>>();
    let king = King::new(PieceColor::White, start, CELL_SIZE);
    let king_moves = king
        .move_set()
        .into_iter()
        .take_while(|mov| mov.special.is_none())
        .map(|mov| mov.to)
        .collect::<IndexSet<_>>();
    assert_eq!(king_moves, moves)
}

#[test]
fn starting_queen() {
    let queen = Queen::new(PieceColor::White, Point::try_from("d1").unwrap(), CELL_SIZE);
    let start = Point::new(3, 0);
    assert_eq!(
        queen.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(4, 0), None, Some(Direction::Right)),
            Movement::new(start, Point::new(5, 0), None, Some(Direction::Right)),
            Movement::new(start, Point::new(6, 0), None, Some(Direction::Right)),
            Movement::new(start, Point::new(7, 0), None, Some(Direction::Right)),
            Movement::new(start, Point::new(4, 1), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(5, 2), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(6, 3), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(7, 4), None, Some(Direction::UpRight)),
            Movement::new(start, Point::new(3, 1), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 2), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 3), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 4), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 5), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 6), None, Some(Direction::Up)),
            Movement::new(start, Point::new(3, 7), None, Some(Direction::Up)),
            Movement::new(start, Point::new(2, 1), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(1, 2), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(0, 3), None, Some(Direction::UpLeft)),
            Movement::new(start, Point::new(2, 0), None, Some(Direction::Left)),
            Movement::new(start, Point::new(1, 0), None, Some(Direction::Left)),
            Movement::new(start, Point::new(0, 0), None, Some(Direction::Left)),
        ])
    )
}

// `linear` tests

#[test]
fn bishop_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(3, 7);
    let bishop = Bishop::new(PieceColor::Black, pos, CELL_SIZE);
    board[pos] = Some(Box::new(bishop));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear().is_some())
    }
}

#[test]
fn rook_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(5, 1);
    let rook = Rook::new(PieceColor::White, pos, CELL_SIZE);
    board[pos] = Some(Box::new(rook));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear().is_some())
    }
}

#[test]
fn queen_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(7, 4);
    let queen = Queen::new(PieceColor::White, pos, CELL_SIZE);
    board[pos] = Some(Box::new(queen));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear().is_some())
    }
}

#[test]
fn knight_non_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(3, 7);
    let knight = Knight::new(PieceColor::Black, pos, CELL_SIZE);
    board[pos] = Some(Box::new(knight));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear().is_none())
    }
}

// `piece_from_char` tests.
// (also test Point limits and Piece::as_any)
#[test]
fn pieces_from_char() {
    piece_from_char('R', PieceColor::White, Point::new(1, 0));
    piece_from_char('B', PieceColor::Black, Point::new(9, 2));
    piece_from_char('Q', PieceColor::Black, Point::new(3, 1));
    piece_from_char('N', PieceColor::White, Point::new(12, -4));
}

#[test]
fn char_rook() {
    assert!(
        piece_from_char('R', PieceColor::White, Point::default())
            .unwrap()
            .as_any()
            .is::<Rook>(),
    );
}
#[test]
fn char_bishop() {
    assert!(
        piece_from_char('B', PieceColor::Black, Point::new(0, 2))
            .unwrap()
            .as_any()
            .is::<Bishop>(),
    );
}
#[test]
fn char_knight() {
    assert!(
        piece_from_char('N', PieceColor::White, Point::new(10, 2))
            .unwrap()
            .as_any()
            .is::<Knight>(),
    );
}
#[test]
fn char_queen() {
    assert!(
        piece_from_char('Q', PieceColor::Black, Point::new(0xa5, isize::MIN))
            .unwrap()
            .as_any()
            .is::<Queen>(),
    );
}

#[test]
#[should_panic]
fn piece_from_char_unknown() {
    assert!(piece_from_char('A', PieceColor::White, Point::new(isize::MAX, 0)).is_some());
}

#[test]
#[should_panic]
fn piece_from_invalid_char() {
    assert!(piece_from_char('K', PieceColor::Black, Point::new(10, -2)).is_some()); // King, but cannot upgrade to king
}

// color
#[test]
fn color_to_bool() {
    assert!(bool::from(PieceColor::White));
    assert!(!bool::from(PieceColor::Black));
    assert!(!bool::from(PieceColor::White.opposite()));
    assert!(bool::from(PieceColor::Black.opposite()));
}

#[test]
fn color_to_mul() {
    assert_eq!(PieceColor::White.sign(), 1);
    assert_eq!(PieceColor::Black.sign(), -1);
    assert_eq!(PieceColor::White.opposite().sign(), -1);
    assert_eq!(PieceColor::Black.opposite().sign(), 1);
}
