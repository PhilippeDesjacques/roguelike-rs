use crate::game::Game;
use crate::rendering::TcodRenderingComponent;
use crate::util::Point;
use tcod::input::Key;
use tcod::RootConsole;

pub trait Updates {
    fn update(&mut self, g: &Game);
    fn render(&self, c: &mut TcodRenderingComponent);
}