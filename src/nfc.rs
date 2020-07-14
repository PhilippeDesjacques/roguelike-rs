use crate::util::Point;
use crate::traits::{Updates, RenderingComponent, MovementComponent};
use crate::game::Game;

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
    fn update(&mut self, g: &Game) {
        self.movement_component.set_key_pressed(g.last_key_pressed());
        self.position = self.movement_component.update(self.position.clone());
    }

    fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(&self.position, self.display_char);
    }
}