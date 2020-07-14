use crate::util::{Bound, Point, Contains};
use crate::traits::MovementComponent;
use rand::{thread_rng, Rng};
use tcod::input::{Key, KeyCode};

pub struct RandomMovementComponent {
    pub windows_bounds: Bound,
    pub key_pressed: Option<Key>,
}

impl MovementComponent for RandomMovementComponent {
    fn new(bound: &Bound) -> RandomMovementComponent {
        RandomMovementComponent{windows_bounds: bound.clone(), key_pressed: None}
    }

    fn update(&self, point: Point) -> Point {
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
    fn new(bound: &Bound) -> InputMovementComponent {
        InputMovementComponent{windows_bounds: bound.clone(), key_pressed: None}
    }

    fn update(&self, point: Point) -> Point {
        //let mut offset = Point{x: point.x, y: point.y};
        let mut offset = Point{x: 0, y: 0};
        match self.key_pressed {
            Some(a) => match a.code {
                KeyCode::Up => offset.y = -1,
                KeyCode::Down => offset.y = 1,
                KeyCode::Left => offset.x = -1,
                KeyCode::Right => offset.x = 1,
                _ => {}
            },
            _ => {}
        }
        match self.windows_bounds.contains(point.offset(&offset)) {
            Contains::DoesContain => offset = point.offset(&offset),
            Contains::DoesNotContain => {return point}
        }
        offset
    }

    fn set_key_pressed(&mut self, k: Option<Key>) {
        self.key_pressed = k;
    }
}