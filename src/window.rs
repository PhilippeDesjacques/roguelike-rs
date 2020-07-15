use crate::util::Bound;
use tcod::Color;
use tcod::console::{Console, Offscreen};

macro_rules! window_component_getters(
    () => {
        fn bounds(&self) -> Bound {
            self.bounds
        }

        fn bg_color(&self) -> Color {
            self.background_color
        }

        fn console(&mut self) -> &mut Offscreen {
            &mut self.console
        }

        fn mut_messages(&mut self) -> &mut Vec<Box<String>> {
            &mut self.messages
        }

        fn messages(&self) -> Vec<Box<String>> {
            self.messages.clone()
        }

        fn max_messages(&self) -> usize {
            self.max_messages
        }
    }
);

macro_rules! window_component_def(
    ($name:ident) => {
        pub struct $name {
            pub console: Offscreen,
            pub background_color: Color,
            bounds: Bound,
            messages: Vec<Box<String>>,
            max_messages: usize,
        }
    }
);

macro_rules! window_component_init(
    ($name:ident, $color:expr, $max_messages:expr) => {
        fn new(bounds: Bound) -> $name {
            let height = bounds.max.y - bounds.min.y + 1;
            let width = bounds.max.x - bounds.min.x + 1;
            let console = Offscreen::new(
                width,
                height,
            );

            $name {
                console,
                background_color: $color,
                bounds,
                messages: vec![],
                max_messages: $max_messages,
            }
        }
    }
);

macro_rules! window_component_create(
    ($name:ident, $max_messages:expr) => {
        window_component_def!($name);

        impl WindowComponent for $name {
            window_component_init!($name, Color::new(0, 0, 0), $max_messages);

            window_component_getters!();
        }
    }
);

pub trait WindowComponent {
    fn new(bounds: Bound) -> Self where Self: Sized;

    fn bounds(&self) -> Bound;
    fn bg_color(&self) -> Color;
    fn console(&mut self) -> &mut Offscreen;

    fn clear(&mut self) {
        let color = self.bg_color();
        let console = self.console();
        console.set_default_background(color);
        console.clear();
    }

    fn print_message(&mut self, x: i32, y: i32, alignment: tcod::TextAlignment, text: &str) {
        let console = self.console();
        console.print_ex(x, y, tcod::console::BackgroundFlag::Set, alignment, text);
    }

    fn mut_messages(&mut self) -> &mut Vec<Box<String>>;
    fn messages(&self) -> Vec<Box<String>>;
    fn max_messages(&self) -> usize;

    fn buffer_message(&mut self, text: &str) {
        let max = self.max_messages();
        let message = text.into();
        let messages = self.mut_messages();

        messages.insert(0, Box::new(message));
        messages.truncate(max);
    }

    fn clear_buffer(&mut self) {
        let max = self.max_messages();
        let messages = self.mut_messages();

        for _ in 0..max {
            messages.insert(0, Box::new(String::new()))
        }
        messages.truncate(max);
    }
}

window_component_create!(StatsWindowComponent, 10);

window_component_create!(InputWindowComponent, 2);

window_component_create!(MessageWindowComponent, 10);

window_component_create!(MapWindowComponent, 10);