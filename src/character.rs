use crate::util::{Point, Contains};
use crate::game::Game;
use tcod::input::KeyCode;
use crate::rendering::TcodRenderingComponent;

pub struct Character {
    pub position: Point,
    pub display_char: char
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Character {
        Character{position: Point{x, y}, display_char: dc}
    }

    pub fn update(&mut self, ks: tcod::input::Key, g: &Game) {
        let mut offset = Point{x: 0, y: 0};
        match ks.code {
            KeyCode::Up => offset.y = -1,
            KeyCode::Down => offset.y = 1,
            KeyCode::Left => offset.x = -1,
            KeyCode::Right => offset.x = 1,
            _ => {}
        }
        match g.windows_bounds.contains(self.position.offset(&offset)) {
            Contains::DoesContain => self.position = self.position.offset(&offset),
            Contains::DoesNotContain => {}
        }
    }

    pub fn render(&self, rendering_component: &mut TcodRenderingComponent) {
        rendering_component.render_object(&self.position, self.display_char);
    }
}