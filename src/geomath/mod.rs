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
pub struct Point<T> {
    /// **x** coordinate of the `Point`.
    pub x: T,
    /// **y** coordinate of the `Point`.
    pub y: T,
}

impl<T> Point<T> {
    /// Constructor of `Point`.
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<isize> {
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

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T> AddAssign for Point<T>
where
    T: AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl<T> Sub<T> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: T) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl<T> Neg for Point<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    #[inline(always)]
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}
impl TryFrom<&str> for Point<isize> {
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
            .ok_or("invalid coords")?
            .0 as isize;

        let y = y.to_string().parse::<isize>()? - 1; // can't be negative because its just 1 char

        (y < Board::SIZE as isize)
            .then_some(Point::new(x, y))
            .ok_or("invalid coords".into())
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Point<isize>> for Point<f32> {
    #[inline]
    fn from(point: Point<isize>) -> Self {
        Self::new(point.x as f32, point.y as f32)
    }
}

impl From<Point<f32>> for Point<isize> {
    #[inline]
    fn from(point: Point<f32>) -> Self {
        Self::new(point.x as isize, point.y as isize)
    }
}
