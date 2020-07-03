use tcod::{RootConsole, Console, BackgroundFlag};
use crate::util::Point;
use tcod::input::Key;

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl TcodRenderingComponent {
    pub fn new(c: RootConsole) -> Self {
        TcodRenderingComponent{console: c}
    }

    pub fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    pub fn render_object(&mut self, position: &Point, symbol: char) {
        self.console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    pub fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }
}