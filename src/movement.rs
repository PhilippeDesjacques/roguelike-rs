use crate::util::{Bound, Point, Contains};
use rand::{thread_rng, Rng};
use tcod::input::{Key, KeyCode};
use crate::game::Game;
use crate::util::XPointRelation::{RightOfPoint, LeftOfPoint, OnPointX};
use crate::util::YPointRelation::{AbovePoint, BelowPoint, OnPointY};
use crate::util::PointEquality::{PointEqual, PointNotEqual};

pub trait MovementComponent {
    fn new(bound: Bound) -> Self where Self: Sized;
    fn update(&self, point: Point, g: &Game) -> Point;
    fn set_key_pressed(&mut self, k: Option<Key>);
}

pub struct RandomMovementComponent {
    pub windows_bounds: Bound,
    pub key_pressed: Option<Key>,
}

impl MovementComponent for RandomMovementComponent {
    fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent{windows_bounds: bound.clone(), key_pressed: None}
    }

    fn update(&self, point: Point, _g: &Game) -> Point {
        let mut offset = Point{x: point.x, y: point.y};
        let mut thr = thread_rng();
        let off_x = thr.gen_range(0, 3) - 1;
        match self.windows_bounds.contains(point.offset_x(off_x)) {
            Contains::DoesContain => offset = offset.offset_x(off_x),
            Contains::DoesNotContain => { return offset; }
        }
        let off_y = thr.gen_range(0, 3) - 1;
        match self.windows_bounds.contains(point.offset_y(off_y)) {
            Contains::DoesContain => offset = offset.offset_y(off_y),
            Contains::DoesNotContain => { return offset; }
        }
        offset
    }

    fn set_key_pressed(&mut self, k: Option<Key>) {
        self.key_pressed = k;
    }
}

pub struct InputMovementComponent {
    pub windows_bounds: Bound,
    pub key_pressed: Option<Key>,
}

impl MovementComponent for InputMovementComponent {
    fn new(bound: Bound) -> InputMovementComponent {
        InputMovementComponent{windows_bounds: bound.clone(), key_pressed: None}
    }

    fn update(&self, point: Point, _g: &Game) -> Point {
        let mut offset = Point{x: point.x, y: point.y};
        //let mut offset = Point{x: 0, y: 0};
        offset = match self.key_pressed {
            Some(a) => match a.code {
                KeyCode::Up => offset.offset_y(-1),
                KeyCode::Down => offset.offset_y(1),
                KeyCode::Left => offset.offset_x(-1),
                KeyCode::Right => offset.offset_x(1),
                _ => offset
            },
            None => offset
        };
        match self.windows_bounds.contains(offset.clone()) {
            Contains::DoesContain => offset,
            Contains::DoesNotContain => point
        }
    }

    fn set_key_pressed(&mut self, k: Option<Key>) {
        self.key_pressed = k;
    }
}

pub struct AggroMovementComponent {
    pub windows_bounds: Bound,
    pub key_pressed: Option<Key>,
}

impl MovementComponent for AggroMovementComponent {
    fn new(bound: Bound) -> AggroMovementComponent {
        AggroMovementComponent{windows_bounds: bound.clone(), key_pressed: None}
    }

    fn update(&self, point: Point, g: &Game) -> Point {
        let char_point = g.character_position();
        let mut offset = Point{x: 0, y: 0};
        match point.compare_x(char_point) {
            RightOfPoint => offset = offset.offset_x(-1),
            LeftOfPoint => offset = offset.offset_x(1),
            OnPointX => {}
        }
        match point.compare_y(char_point) {
            BelowPoint => offset = offset.offset_y(1),
            AbovePoint => offset = offset.offset_y(-1),
            OnPointY => {}
        }
        match point.offset(&offset).compare(char_point) {
            PointEqual => point,
            PointNotEqual => {
                match self.windows_bounds.contains(point.offset(&offset)) {
                    Contains::DoesContain => point.offset(&offset),
                    Contains::DoesNotContain => point
                }
            }
        }
    }

    fn set_key_pressed(&mut self, k: Option<Key>) {
        self.key_pressed = k;
    }
}
