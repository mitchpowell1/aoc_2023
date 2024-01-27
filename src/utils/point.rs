use crate::utils::direction::Direction;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Point(pub i32, pub i32);

impl Add<(i8, i8)> for Point {
    type Output = Point;
    fn add(self, other: (i8, i8)) -> Self {
        Point(self.0 + other.0 as i32, self.1 + other.1 as i32)
    }
}

impl AddAssign<(i8, i8)> for Point {
    fn add_assign(&mut self, other: (i8, i8)) {
        self.0 += other.0 as i32;
        self.1 += other.1 as i32;
    }
}

impl Add<Direction> for Point {
    type Output = Point;
    fn add(self, other: Direction) -> Self {
        self + other.get_offset()
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        *self += other.get_offset();
    }
}

pub trait InBounds {
    fn is_in_bounds(&self, p: Point) -> bool;
}

impl<T> InBounds for Vec<Vec<T>> {
    fn is_in_bounds(&self, Point(i, j): Point) -> bool {
        i >= 0 && j >= 0 && (i as usize) < self.len() && (j as usize) < self[0].len()
    }
}

impl SubAssign<(i8, i8)> for Point {
    fn sub_assign(&mut self, other: (i8, i8)) {
        self.0 -= other.0 as i32;
        self.1 -= other.1 as i32;
    }
}

impl MulAssign<(i8, i8)> for Point {
    fn mul_assign(&mut self, other: (i8, i8)) {
        self.0 *= other.0 as i32;
        self.1 *= other.1 as i32;
    }
}
