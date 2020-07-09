use crate::util::{Point, Contains};
use crate::traits::{Updates, RenderingComponent, MovementComponent};
use crate::game::Game;
use rand::{Rng, thread_rng};

pub struct NFC {
    pub position: Point,
    pub display_char: char,
    pub movement_component: Box<dyn MovementComponent>,
}

impl NFC {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<dyn MovementComponent>) -> NFC {
        NFC{position: Point{x, y}, display_char: dc, movement_component: mc}
    }
}

impl Updates for NFC {
    fn update(&mut self) {
        let mut thr = thread_rng();
        let off_x = thr.gen_range(0, 3) - 1;
        match self.movement_component.bound().contains(self.position.offset_x(off_x)) {
            Contains::DoesContain => self.position = self.position.offset_x(off_x),
            Contains::DoesNotContain => {}
        }
        let off_y = thr.gen_range(0, 3) - 1;
        match self.movement_component.bound().contains(self.position.offset_y(off_y)) {
            Contains::DoesContain => self.position = self.position.offset_y(off_y),
            Contains::DoesNotContain => {}
        }
    }

    fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(&self.position, self.display_char);
    }
}