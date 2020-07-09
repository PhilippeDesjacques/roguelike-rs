use crate::game::Game;
use crate::util::{Point, Bound};
use tcod::input::Key;
use tcod::RootConsole;

pub trait Updates {
    fn update(&mut self);
    fn render(&self, c: &mut Box<dyn RenderingComponent>);
}

pub trait RenderingComponent {
    fn new(console: RootConsole) -> Self where Self: Sized;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, p: &Point, c: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn console(&self) -> &RootConsole;
}

pub trait MovementComponent {
    fn new(bound: &Bound) -> Self where Self: Sized;
    fn update(&self, point: Point) -> Point;
    fn bound(&self) -> &Bound;
}