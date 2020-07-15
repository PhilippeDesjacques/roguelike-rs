use tcod::{RootConsole, Console, BackgroundFlag, TextAlignment};
use crate::util::Point;
use tcod::input::Key;
use crate::window::WindowComponent;

pub trait RenderingComponent {
    fn new(console: RootConsole) -> Self where Self: Sized;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, p: &Point, c: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn console(&self) -> &RootConsole;
    fn attach_window(&mut self, window: &mut Box<dyn WindowComponent>);
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

    fn attach_window(&mut self, window: &mut Box<dyn WindowComponent>) {
        window.clear();
        let mut line = 0;
        let bounds = window.bounds();
        let messages = window.messages();

        for message in messages.iter() {
            window.print_message(0, line, TextAlignment::Left, (**message).as_str());
            line = line + 1;
        }

        let console = window.console();
        tcod::console::blit(&*console, (0, 0), (bounds.width(), bounds.height()), &mut self.console, (bounds.min.x, bounds.min.y), 1f32, 1f32);
    }
}