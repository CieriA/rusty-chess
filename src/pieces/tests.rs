use crate::prelude::*;
use indexmap::IndexSet;

#[test]
fn pawn() {
    let pawn = Pawn::new(Color::Black, Point::from("e7"));
    assert_eq!(
        pawn.move_set(),
        IndexSet::from([
            Movement::new(Point::new(4, 6), Point::new(4, 5), None, Some(Direction::Up)),
            Movement::new(Point::new(4, 6), Point::new(4, 4), Some(SpecialMove::DoublePawn), Some(Direction::Up)),
            Movement::new(Point::new(4, 6), Point::new(3, 5), Some(SpecialMove::PawnEat), Some(Direction::UpRight)),
            Movement::new(Point::new(4, 6), Point::new(5, 5), Some(SpecialMove::PawnEat), Some(Direction::UpLeft))
        ])
    );
}

#[test]
fn bishop() {
    let bishop = Bishop::new(Color::White, Point::from("b2"));
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
    let bishop = Bishop::new(Color::Black, Point::from("g6"));
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
    let rook = Rook::new(Color::Black, Point::from("h8"));
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
    let knight = Knight::new(Color::White, Point::from("d4"));
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
    let knight = Knight::new(Color::White, Point::from("b1"));
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
    let king = King::new(Color::Black, Point::from("e8"));
    let start = Point::new(4, 7);
    assert_eq!(
        king.move_set(),
        IndexSet::from([
            Movement::new(start, Point::new(3, 7), None, Some(Direction::Left)),
            Movement::new(start, Point::new(5, 7), None, Some(Direction::Right)),
            Movement::new(start, Point::new(4, 6), None, Some(Direction::Down)),
            Movement::new(start, Point::new(3, 6), None, Some(Direction::DownLeft)),
            Movement::new(start, Point::new(5, 6), None, Some(Direction::DownRight)),
            Movement::new(start, Point::new(2, 7), Some(SpecialMove::LongCastle), Some(Direction::Left)),
            Movement::new(start, Point::new(6, 7), Some(SpecialMove::ShortCastle), Some(Direction::Right)),
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
    let king = King::new(Color::White, start);
    let king_moves = king
        .move_set()
        .into_iter()
        .take_while(|mov| mov.special.is_none())
        .map(|mov| mov.to)
        .collect::<IndexSet<_>>();
    assert_eq!(
        king_moves,
        moves
    )
}


#[test]
fn starting_queen() {
    let queen = Queen::new(Color::White, Point::from("d1"));
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
    let bishop = Bishop::new(Color::Black, pos);
    board[pos] = Some(Box::new(bishop));
    
    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear())
    }
}

#[test]
fn rook_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(5, 1);
    let rook = Rook::new(Color::White, pos);
    board[pos] = Some(Box::new(rook));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear())
    }
}

#[test]
fn queen_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(7, 4);
    let queen = Queen::new(Color::White, pos);
    board[pos] = Some(Box::new(queen));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear())
    }
}

#[test]
#[should_panic]
fn knight_non_linearity() {
    let mut board = Board::empty();
    let pos = Point::new(3, 7);
    let knight = Knight::new(Color::Black, pos);
    board[pos] = Some(Box::new(knight));

    for mov in board.filtered_move_set(pos) {
        assert!(mov.linear())
    }
}
