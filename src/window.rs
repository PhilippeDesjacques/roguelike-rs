use crate::util::Bound;
use tcod::Color;
use tcod::console::{Console, Offscreen};

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
}

pub struct StatsWindowComponent {
    pub console: Offscreen,
    pub background_color: Color,
    bounds: Bound,
}

impl WindowComponent for StatsWindowComponent {
    fn new(bounds: Bound) -> StatsWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Offscreen::new(
            width,
            height,
        );

        let red = Color::new(255, 0, 0);
        StatsWindowComponent {
            console,
            background_color: red,
            bounds
        }
    }

    fn bounds(&self) -> Bound {
        self.bounds
    }

    fn bg_color(&self) -> Color {
        self.background_color
    }

    fn console(&mut self) -> &mut Offscreen {
        &mut self.console
    }
}

pub struct InputWindowComponent {
    pub console: Offscreen,
    pub background_color: Color,
    bounds: Bound,
}

impl WindowComponent for InputWindowComponent {
    fn new(bounds: Bound) -> InputWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Offscreen::new(
            width,
            height,
        );

        let green = Color::new(0, 255, 0);
        InputWindowComponent {
            console,
            background_color: green,
            bounds
        }
    }

    fn bounds(&self) -> Bound {
        self.bounds
    }

    fn bg_color(&self) -> Color {
        self.background_color
    }

    fn console(&mut self) -> &mut Offscreen {
        &mut self.console
    }
}

pub struct MessageWindowComponent {
    pub console: Offscreen,
    pub background_color: Color,
    bounds: Bound,
}

impl WindowComponent for MessageWindowComponent {
    fn new(bounds: Bound) -> MessageWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Offscreen::new(
            width,
            height,
        );

        let blue = Color::new(0, 0, 255);
        MessageWindowComponent {
            console,
            background_color: blue,
            bounds
        }
    }

    fn bounds(&self) -> Bound {
        self.bounds
    }

    fn bg_color(&self) -> Color {
        self.background_color
    }

    fn console(&mut self) -> &mut Offscreen {
        &mut self.console
    }
}

pub struct MapWindowComponent {
    pub console: Offscreen,
    pub background_color: Color,
    bounds: Bound,
}

impl WindowComponent for MapWindowComponent {
    fn new(bounds: Bound) -> MapWindowComponent {
        let height = bounds.max.y - bounds.min.y + 1;
        let width = bounds.max.x - bounds.min.x + 1;
        let console = Offscreen::new(
            width,
            height,
        );

        let purple = Color::new(255, 0, 255);
        MapWindowComponent {
            console,
            background_color: purple,
            bounds
        }
    }

    fn bounds(&self) -> Bound {
        self.bounds
    }

    fn bg_color(&self) -> Color {
        self.background_color
    }

    fn console(&mut self) -> &mut Offscreen {
        &mut self.console
    }
}