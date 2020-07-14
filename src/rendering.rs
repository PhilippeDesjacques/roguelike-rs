use tcod::{RootConsole, Console, BackgroundFlag};
use crate::util::Point;
use tcod::input::Key;

pub trait RenderingComponent {
    fn new(console: RootConsole) -> Self where Self: Sized;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, p: &Point, c: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn console(&self) -> &RootConsole;
}

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(c: RootConsole) -> TcodRenderingComponent {
        TcodRenderingComponent{console: c}
    }

    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: &Point, symbol: char) {
        self.console.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }

    fn console(&self) -> &RootConsole {
        &self.console
    }
}