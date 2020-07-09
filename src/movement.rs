use crate::util::{Bound, Point, Contains};
use crate::traits::MovementComponent;
use rand::{thread_rng, Rng};

pub struct RandomMovementComponent {
    pub windows_bounds: Bound,
}

impl MovementComponent for RandomMovementComponent {
    fn new(bound: &Bound) -> RandomMovementComponent {
        RandomMovementComponent{windows_bounds: bound.clone()}
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point{x: point.x, y: point.y};
        let mut thr = thread_rng();
        let off_x = thr.gen_range(0, 3) - 1;
        match self.windows_bounds.contains(offset.offset_x(off_x)) {
            Contains::DoesContain => offset = offset.offset_x(off_x),
            Contains::DoesNotContain => { return point; }
        }
        let off_y = thr.gen_range(0, 3) - 1;
        match self.windows_bounds.contains(offset.offset_y(off_y)) {
            Contains::DoesContain => offset = offset.offset_y(off_y),
            Contains::DoesNotContain => { return point; }
        }
        offset
    }

    fn bound(&self) -> &Bound {
        &self.windows_bounds
    }
}