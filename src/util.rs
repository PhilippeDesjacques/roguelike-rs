use crate::util::Contains::{DoesNotContain, DoesContain};
use crate::util::XPointRelation::{RightOfPoint, LeftOfPoint, OnPointX};
use crate::util::YPointRelation::{AbovePoint, BelowPoint, OnPointY};
use crate::util::PointEquality::{PointEqual, PointNotEqual};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point{x: self.x + offset, y: self.y}
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point{x: self.x, y: self.y + offset}
    }

    pub fn offset(&self, offset: &Point) -> Point {
        Point{x: self.x + offset.x, y: self.y + offset.y}
    }

    pub fn compare_x(&self, other: &Point) -> XPointRelation {
        if self.x < other.x {
            LeftOfPoint
        } else if self.x > other.x {
            RightOfPoint
        } else {
            OnPointX
        }
    }

    pub fn compare_y(&self, other: &Point) -> YPointRelation {
        if self.y < other.y {
            BelowPoint
        } else if self.y > other.y {
            AbovePoint
        } else {
            OnPointY
        }
    }

    pub fn compare(&self, other: &Point) -> PointEquality {
        if self.x == other.x && self.y == other.y {
            PointEqual
        } else {
            PointNotEqual
        }
    }
}

pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
}

pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY,
}

pub enum PointEquality {
    PointEqual,
    PointNotEqual,
}

#[derive(Debug)]
pub enum Contains {
    DoesContain,
    DoesNotContain
}

#[derive(Clone, Copy, Debug)]
pub struct Bound {
    pub min: Point,
    pub max: Point
}

impl Bound {

    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Bound{
            min: Point{x: min_x, y: min_y},
            max: Point{x: max_x, y: max_y},
        }
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x + 1
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y + 1
    }

    pub fn contains(&self, point: Point) -> Contains {
        if
        point.x > self.min.x &&
            point.x < self.max.x &&
            point.y > self.min.y &&
            point.y < self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}