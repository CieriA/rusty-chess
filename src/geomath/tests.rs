use crate::prelude::*;
use std::collections::HashSet;

#[test]
fn point() {
    assert_eq!(Point::default(), Point::new(0, 0));

    // declaration
    let p = Point::new(-7, 0);

    // Def add
    assert_eq!(p + Point::default(), p);
}
#[test]
fn point_from_tuple() {
    assert_eq!(Point::from((3, 7)), Point::new(3, 7));
    assert_eq!(Point::new(2, -8), (2, -8).into());
    assert_eq!(Point::default(), (0, 0).into());
    assert_eq!(Point::new(-10, 10), Point::from((-10, 10)));
}

#[test]
fn point_from_str() {
    assert_eq!(
        // first
        Point::try_from("a1").unwrap(),
        Point::new(0, 0)
    );
    assert_eq!(
        // last
        Point::try_from("h8").unwrap(),
        Point::new(7, 7)
    );

    assert_eq!(Point::try_from("e4").unwrap(), Point::new(4, 3));
    assert_eq!(Point::try_from("B7").unwrap(), Point::new(1, 6));
    assert_eq!(Point::try_from("h5").unwrap(), Point::new(7, 4));
    assert_eq!(
        Point::try_from(String::from("A3").as_str()).unwrap(),
        Point::new(0, 2)
    )
}
#[test]
#[should_panic]
fn p_from_str_num_out_of_bounds() {
    let _ = Point::try_from("B9").unwrap();
}
#[test]
#[should_panic]
fn point_from_str_letter_out_of_bounds() {
    let _ = Point::try_from("j3").unwrap();
}
#[test]
#[should_panic]
fn point_from_str_invalid_syntax() {
    let _ = Point::try_from("c").unwrap();
}
#[test]
#[should_panic]
fn point_from_str_empty() {
    let _ = Point::try_from("").unwrap();
}

#[test]
fn point_add() {
    assert_eq!(Point::new(1, 2) + Point::new(2, 1), Point::new(3, 3));
    let p = Point::new(3, 4);
    let mut p2 = p;
    p2 += Point::new(1, -1);
    assert_ne!(p, p2);
    assert_eq!(p + Point::new(1, -1), p2);
}
#[test]
fn point_sub() {
    assert_eq!(Point::new(3, 2) - Point::new(2, 3), Point::new(1, -1));
    assert_eq!(Point::new(7, -2) - Point::default(), Point::new(7, -2));
}
#[test]
fn point_neg() {
    assert_eq!(Point::default(), -Point::default());

    assert_eq!(Point::new(3, 12), -Point::new(-3, -12));

    assert_eq!(Point::new(-1, 3), -Point::new(1, -3));
}

#[test]
fn knight_rots() {
    let rots = Point::new(1, 2).rotations();
    let res = HashSet::from([
        (Point::new(1, 2), None),
        (Point::new(-1, 2), None),
        (Point::new(1, -2), None),
        (Point::new(-1, -2), None),
        (Point::new(2, 1), None),
        (Point::new(-2, 1), None),
        (Point::new(2, -1), None),
        (Point::new(-2, -1), None),
    ]);
    assert_eq!(rots, res);
}

#[test]
fn opposite() {
    // NOTE: Direction::from is tested with Point::rotations ('cause its used only there)
    assert_eq!(Direction::Up.opposite_if(false), Direction::Up);
    assert_eq!(Direction::UpLeft.opposite_if(true), Direction::DownRight);
    assert_eq!(Direction::UpRight.opposite(), Direction::DownLeft);
    assert_eq!(Direction::Right.opposite(), Direction::Left,);
    assert_eq!(Direction::Down.opposite(), Direction::Up);
}

#[test]
fn bishop_rots() {
    let rots = Point::new(1, 1).rotations();
    let res = HashSet::from([
        (Point::new(1, 1), Some(Direction::UpRight)),
        (Point::new(-1, 1), Some(Direction::UpLeft)),
        (Point::new(1, -1), Some(Direction::DownRight)),
        (Point::new(-1, -1), Some(Direction::DownLeft)),
    ]);
    assert_eq!(rots, res);
}

#[test]
fn rook_rots() {
    let rots = Point::new(0, 1).rotations();
    let res = HashSet::from([
        (Point::new(0, 1), Some(Direction::Up)),
        (Point::new(1, 0), Some(Direction::Right)),
        (Point::new(0, -1), Some(Direction::Down)),
        (Point::new(-1, 0), Some(Direction::Left)),
    ]);
    assert_eq!(rots, res);
}

#[test]
fn king_rots() {
    let rots = Point::all_around(1);
    let res = HashSet::from([
        (Point::new(0, 1), Some(Direction::Up)),
        (Point::new(1, 1), Some(Direction::UpRight)),
        (Point::new(1, 0), Some(Direction::Right)),
        (Point::new(1, -1), Some(Direction::DownRight)),
        (Point::new(0, -1), Some(Direction::Down)),
        (Point::new(-1, -1), Some(Direction::DownLeft)),
        (Point::new(-1, 0), Some(Direction::Left)),
        (Point::new(-1, 1), Some(Direction::UpLeft)),
    ]);
    assert_eq!(rots, res);
}
