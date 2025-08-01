use crate::geomath::{Point, rotation::Direction};

/// Moves that can happen under special circumstances
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SpecialMove {
    /// When the pawn cannot eat by going straight (it never can).
    CannotEat,

    /// A pawn can eat in diagonal only.
    ///
    /// En Passant is a subcategory of this
    PawnEat,

    /// You can move a pawn by 2 squares instead of 1
    /// only if it has never been moved.
    DoublePawn,

    /// When the King and the furthest Rook have never been moved,
    /// and there are no pieces between them, you can move the King by (-3, 0) and the Rook
    /// next to it in the opposite direction.
    LongCastle,

    /// When the King and the nearest Rook have never been moved,
    /// and there are no pieces between them, you can move the King by (2, 0) and the Rook
    /// next to it in the opposite direction.
    ShortCastle,
}

/// A movement of a piece
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Movement {
    /// the square the piece starts
    pub from: Point,
    /// the square the piece arrives
    pub to: Point,
    /// type of move
    pub special: Option<SpecialMove>,
    /// the direction the piece is going to when talking about a
    /// Bishop/Rook/Queen who need to stop when colliding.
    ///
    /// `None` when the move doesn't comprehend these pieces.
    pub direction: Option<Direction>,
}
impl Movement {
    #[inline]
    pub const fn new(
        from: Point,
        to: Point,
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

    pub fn linear(&self) -> Option<Point> {
        let step = self.to - self.from;
        (step.x == 0 || step.y == 0 || step.x.abs() == step.y.abs())
            .then_some(Point::new(step.x.signum(), step.y.signum()))
    }
}
