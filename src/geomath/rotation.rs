use crate::geomath::Point;

/// 90 degrees rotations
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum Direction {
    /// `Up` (**0deg**)
    #[default]
    Up,

    /// `Up-right` (**45deg**)
    UpRight,

    /// `Right` (**90deg**)
    Right,

    /// `Down-right` (**135deg**)
    DownRight,

    /// `Down` (**180deg**)
    Down,

    /// `Down-left` (**225deg**)
    DownLeft,

    /// `Left` (**270deg**)
    Left,

    /// `Up-left` (**315deg**)
    UpLeft,
}

impl Direction {
    /// Returns the opposite direction
    #[inline]
    pub(crate) const fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::UpRight => Self::DownLeft,
            Self::Right => Self::Left,
            Self::DownRight => Self::UpLeft,
            Self::Down => Self::Up,
            Self::DownLeft => Self::UpRight,
            Self::Left => Self::Right,
            Self::UpLeft => Self::DownRight,
        }
    }
    /// Returns the opposite direction only if the condition is true
    #[inline]
    pub(crate) const fn opposite_if(&self, cond: bool) -> Self {
        if cond {
            self.opposite()
        } else {
            *self
        }
    }
}

impl From<Point> for Option<Direction> {
    fn from(value: Point) -> Self {
        let Point { x, y } = value;

        match (x, y) {
            (0, 0) => None, // We need to check first that both aren't 0 or next arms won't work

            (0, y) =>
            // x is 0
            {
                Some(Direction::Up.opposite_if(y.is_negative()))
            }

            (x, 0) =>
            // y is 0
            {
                Some(Direction::Right.opposite_if(x.is_negative()))
            }

            (x, y) if x == y =>
            // Both same and we check the signs
            {
                Some(Direction::UpRight.opposite_if(x.is_negative()))
            }

            (x, y) if x.abs() == y.abs() =>
            // Both different signs because of the previous arm
            {
                Some(Direction::DownRight.opposite_if(x.is_negative()))
            }

            _ => None, // We don't need directions for the knight
        }
    }
}
