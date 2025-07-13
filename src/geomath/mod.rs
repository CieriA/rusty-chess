//! A way to index a 2D matrix.

pub mod rotation;
#[cfg(test)]
mod tests;

use crate::chessboard::Board;
use crate::geomath::rotation::Direction;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

/// A **point** (and also a **vector**) in a **2D space**.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    /// **x** coordinate of the `Point`.
    pub x: isize,
    /// **y** coordinate of the `Point`.
    pub y: isize,
}

impl Point {
    /// Constructor of `Point`.
    #[inline(always)]
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn rotations(self) -> HashSet<(Self, Option<Direction>)> {
        let Point { x, y } = self;
        let iter = [(x, y), (-x, y)].map(Self::from).into_iter();
        let iter = iter.clone().chain(iter.map(Neg::neg));

        let set: HashSet<Self> = iter
            .clone()
            .chain(iter.map(|Point { x, y }| Self::new(y, x)))
            .collect();

        set.into_iter().map(|point| (point, point.into())).collect()
    }
    /// Returns all the cells in a square (l = 2 * offset + 1) around (0, 0)
    pub fn all_around(offset: isize) -> HashSet<(Self, Option<Direction>)> {
        Self::new(offset, offset)
            .rotations()
            .into_iter()
            .chain(Self::new(offset, 0).rotations())
            .collect()
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
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
impl TryFrom<&str> for Point {
    type Error = Box<dyn Error>;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s: Vec<char> = s.chars().collect();

        if s.len() != 2 {
            return Err("invalid length".into());
        }

        let x = s[0].to_ascii_uppercase();
        let y = s[1].to_ascii_uppercase();

        if !x.is_ascii_alphabetic() || !y.is_ascii_alphanumeric() || !y.is_numeric() {
            return Err("invalid coords".into());
        }

        let x = ('A'..='Z')
            .take(Board::SIZE)
            .enumerate()
            .find(|(_, c)| *c == x)
            .ok_or("invalid coords")?.0 as isize;

        let y = y.to_string().parse::<isize>()? - 1; // can't be negative because its just 1 char
        
        (y < Board::SIZE as isize)
            .then_some(Point::new(x, y))
            .ok_or("invalid coords".into())
    }
}

impl Display for Point {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
