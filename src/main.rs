mod util;
mod game;
mod rendering;
mod movement;
mod actor;
mod window;

extern crate tcod;

use tcod::input::KeyCode;
use crate::game::Game;
use crate::actor::Actor;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::heroine(40, 25, game.map_window.bounds());
    let mut npcs: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.map_window.bounds())),
        Box::new(Actor::cat(40, 25, game.map_window.bounds())),
        Box::new(Actor::kobold(20, 20, game.map_window.bounds())),
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
