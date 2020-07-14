use crate::util::Point;
use crate::game::Game;
use crate::traits::{RenderingComponent, MovementComponent};

pub struct Character {
    pub position: Point,
    pub display_char: char,
    pub movement_component: Box<dyn MovementComponent>,
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<dyn MovementComponent>) -> Character {
        Character{position: Point{x, y}, display_char: dc, movement_component: mc}
    }

    pub fn update(&mut self, g: &Game) {
        self.movement_component.set_key_pressed(g.last_key_pressed());
        self.position = self.movement_component.update(self.position.clone());
    }

    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(&self.position, self.display_char);
    }
}