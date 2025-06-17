//! A way to index a 2D matrix.

#[cfg(test)]
mod tests;
pub(crate) mod rotation;

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};
use std::collections::HashSet;
use crate::chessboard::Board;
use crate::geomath::rotation::Direction;

/// A **point** (and also a **vector**) in a **2D space**.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Point {
    /// **x** coordinate of the `Point`.
    pub(crate) x: isize,
    /// **y** coordinate of the `Point`.
    pub(crate) y: isize,
}

impl Point {
    /// Constructor of `Point`.
    #[inline(always)]
    pub(crate) const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    
    pub(crate) fn rotations(self) -> HashSet<(Self, Option<Direction>)> {
        let Point { x, y } = self;
        let iter = [(x, y), (-x, y)].map(Point::from).into_iter();
        let iter = iter
            .clone()
            .chain(iter.map(Neg::neg));
        
        let set: HashSet<Self> = iter
            .clone()
            .chain(iter.map(|Point { x, y }| Point::new(y, x)))
            .collect();
        
        set
            .into_iter()
            .map(|point| (point, point.into()))
            .collect()
    }
    /// Returns all the cells in a square (l = 2 * offset + 1) around (0, 0)
    pub(crate) fn all_around(offset: isize) -> HashSet<(Self, Option<Direction>)> {
        Self::new(offset, offset)
            .rotations()
            .into_iter()
            .chain(
                Self::new(offset, 0)
                    .rotations()
            )
            .collect()
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add<isize> for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: isize) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl AddAssign<isize> for Point {
    #[inline]
    fn add_assign(&mut self, rhs: isize) {
        self.x += rhs;
        self.y += rhs;
    }
}
impl Mul<isize> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: isize) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl MulAssign<isize> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl Sub<isize> for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: isize) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl Neg for Point {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<(isize, isize)> for Point {
    #[inline(always)]
    fn from((x, y): (isize, isize)) -> Self {
        Self::new(x, y)
    }
}
impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let s: Vec<char> = s.chars().collect();
        assert_eq!(s.len(), 2);
        let x = s[0].to_ascii_uppercase();
        let y = s[1].to_ascii_uppercase();
        assert!(
            x.is_ascii_alphabetic() &&
            y.is_ascii_alphanumeric() &&
            y.is_numeric(),
            "Invalid coords: {:?}", s
        );
        let x = ('A'..='Z')
            .take(Board::SIZE)
            .enumerate()
            .find(|(_, c)| *c == x)
            .unwrap().0 as isize;
        let y = format!("{}", y).parse::<isize>().unwrap() - 1;
        assert!(y < Board::SIZE as isize);
        Point::new(x, y)
    }
}

impl Display for Point {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
