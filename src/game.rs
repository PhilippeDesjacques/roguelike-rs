use crate::util::Bound;
use crate::traits::{RenderingComponent, Updates};
use crate::character::Character;
use crate::rendering::TcodRenderingComponent;
use tcod::RootConsole;
use tcod::input::Key;

pub struct Game {
    pub exit: bool,
    pub windows_bounds: Bound,
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
}

impl Game {
    pub fn new(b: Bound) -> Game {
        let root = RootConsole::initializer().size(&b.max.x + 1, &b.max.y + 1).title("libtcod Rust Tutorial").init();
        let rc = Box::new(TcodRenderingComponent::new(root)) as Box<dyn RenderingComponent>;
        Game{exit: false, windows_bounds: b, rendering_component: rc}
    }

    pub fn render(&mut self, npcs: &Vec<Box<dyn Updates>>, c: &Character) {
        self.rendering_component.before_render_new_frame();
        for i in npcs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Box<dyn Updates>>, c: &mut Character, keypress: tcod::input::Key) {
        c.update(keypress, self);
        for i in npcs.iter_mut() {
            i.update();
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rendering_component.wait_for_keypress()
    }

    pub fn bound(&self) -> &Bound {
        &self.windows_bounds
    }
}