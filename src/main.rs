mod util;
mod game;
mod rendering;
mod movement;
mod actor;

extern crate tcod;

use tcod::input::KeyCode;
use crate::util::{Point, Bound};
use crate::game::Game;
use crate::actor::Actor;

fn main() {
    let mut game = Game::new(Bound{min: Point{x: 0, y: 0}, max: Point{x: 79, y: 49}});
    let mut c = Actor::heroine(40, 25, game.bound());
    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.bound())),
        Box::new(Actor::cat(40, 25, game.bound())),
        Box::new(Actor::kobold(20, 20, game.bound())),
    ];
    game.render(&npcs, &c);
    while !(game.rendering_component.console().window_closed() || game.exit) {
        let key = game.wait_for_keypress();
        println!("Pressed key: {:?}", key);
        match key.code {
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c);
        game.render(&npcs, &c);
    }
}
