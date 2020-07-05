mod util;
mod game;
mod traits;
mod character;
mod nfc;
mod rendering;

extern crate tcod;

use tcod::input::KeyCode;
use crate::util::{Point, Bound};
use crate::game::Game;
use crate::traits::Updates;
use crate::character::Character;
use crate::nfc::NFC;

fn main() {
    let mut game = Game::new(Bound{min: Point{x: 0, y: 0}, max: Point{x: 79, y: 49}});
    let mut c = Character::new(40, 25, '@');
    let mut npcs: Vec<Box<dyn Updates>> = vec![
        Box::new(NFC::new(10, 10, 'd')) as Box<dyn Updates>,
        Box::new(NFC::new(40, 25, 'c')) as Box<dyn Updates>,
    ];
    game.render(&npcs, &c);
    while !(game.rendering_component.console().window_closed() || game.exit) {
        let key = game.wait_for_keypress();
        println!("Pressed key: {:?}", key);
        match key.code {
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        game.update(&mut npcs, &mut c, key);
        game.render(&npcs, &c);
    }
}
