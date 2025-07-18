use crate::geomath::Point;
use crate::geomath::rotation::Direction;
use std::cmp::Ordering;

/// Moves that can happen under special circumstances
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SpecialMove {
    /// When the pawn cannot eat by going straight (it never can).
    CannotEat,

    /// A pawn can eat in diagonal only.
    ///
    /// En Passant is a sub-category of this
    PawnEat,

    /// You can move a pawn by 2 squares instead of 1
    /// only if it has never been moved.
    DoublePawn,

    /// When the King and the furthest Rook have never been moved,
    /// and there are no piece between them, you can move the King by (-3, 0) and the Rook
    /// next to it in the opposite direction.
    LongCastle,

    /// When the King and the nearest Rook have never been moved,
    /// and there are no piece between them, you can move the King by (2, 0) and the Rook
    /// next to it in the opposite direction.
    ShortCastle,
}

/// A movement of a piece
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Movement {
    /// the square the piece starts
    pub from: Point<isize>,
    /// the square the piece arrives
    pub to: Point<isize>,
    /// type of move
    pub special: Option<SpecialMove>,
    /// going direction when talking about a Bishop/Rook/Queen
    /// who need to stop when colliding.
    ///
    /// `None` when the move doesn't comprehend these pieces.
    pub direction: Option<Direction>,
}
impl Movement {
    #[inline]
    pub const fn new(
        from: Point<isize>,
        to: Point<isize>,
        special: Option<SpecialMove>,
        direction: Option<Direction>,
    ) -> Self {
        Self {
            from,
            to,
            special,
            direction,
        }
    }

    pub fn linear(&self) -> Option<Point<isize>> {
        let step = self.to - self.from;
        if step.x == 0 || step.y == 0 || step.x.abs() == step.y.abs() {
            let x = match step.x.cmp(&0) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            let y = match step.y.cmp(&0) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            Some(Point::new(x, y))
        } else {
            None
        }
    }
}
