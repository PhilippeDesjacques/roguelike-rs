use crate::util::{Bound, Point};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};
use tcod::RootConsole;
use tcod::input::Key;
use crate::actor::Actor;

pub struct Game {
    pub exit: bool,
    pub windows_bounds: Bound,
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
    last_key_pressed: Option<Key>,
    character_position: Point,
}

impl Game {
    pub fn new(b: Bound) -> Game {
        let root = RootConsole::initializer().size(&b.max.x + 1, &b.max.y + 1).title("libtcod Rust Tutorial").init();
        let rc = Box::new(TcodRenderingComponent::new(root)) as Box<dyn RenderingComponent>;
        Game{exit: false, windows_bounds: b, rendering_component: rc, last_key_pressed: None, character_position: Point{x: 0, y: 0}}
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        for i in npcs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
        c.update(self);
        self.character_position = c.position.clone();
        for i in npcs.iter_mut() {
            i.update(self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        self.set_last_key_pressed(Some(ks));
        ks
    }

    pub fn bound(&self) -> &Bound {
        &self.windows_bounds
    }

    pub fn set_last_key_pressed(&mut self, last: Option<Key>) {
        self.last_key_pressed = last;
    }

    pub fn last_key_pressed(&self) -> Option<Key> {
        self.last_key_pressed
    }

    pub fn character_position(&self) -> &Point {
        &self.character_position
    }
}