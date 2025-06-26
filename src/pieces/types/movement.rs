use std::cmp::Ordering;
use crate::geomath::Point;
use crate::geomath::rotation::Direction;

/// Moves that can happen under special circumstances
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum SpecialMove {
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
pub(crate) struct Movement {
    /// the square the piece starts
    pub(crate) from: Point,
    /// the square the piece arrives
    pub(crate) to: Point,
    /// type of move
    pub(crate) special: Option<SpecialMove>,
    /// going direction when talking about a Bishop/Rook/Queen
    /// who need to stop when colliding.
    ///
    /// `None` when the move doesn't comprehend these pieces.
    pub(crate) direction: Option<Direction>,
}
impl Movement {
    #[inline]
    pub(crate) const fn new(from: Point, to: Point, special: Option<SpecialMove>, direction: Option<Direction>) -> Self {
        Self { from, to, special, direction }
    }
    
    pub(crate) fn linear(&self) -> Option<Point> {
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
