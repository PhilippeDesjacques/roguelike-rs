mod util;
mod game;
mod traits;
mod character;
mod nfc;
mod rendering;

extern crate tcod;

use tcod::RootConsole;
use tcod::input::KeyCode;
use crate::util::{Point, Bound};
use crate::game::Game;
use crate::traits::{Updates, RenderingComponent};
use crate::character::Character;
use crate::nfc::NFC;
use crate::rendering::TcodRenderingComponent;

fn main() {
    let windows_bounds = Bound{min: Point{x: 0, y: 0}, max: Point{x: 79, y: 49}};
    let mut game = Game{exit: false, windows_bounds};
    let root = RootConsole::initializer().size(&game.windows_bounds.max.x + 1, &game.windows_bounds.max.y + 1).title("libtcod Rust Tutorial").init();
    let mut rendering_component = Box::new(TcodRenderingComponent::new(root)) as Box<dyn RenderingComponent>;
    let mut c = Character::new(40, 25, '@');
    let d = Box::new(NFC::new(10, 10, 'd')) as Box<dyn Updates>;
    let ct = Box::new(NFC::new(40, 25, 'c')) as Box<dyn Updates>;
    let mut npcs: Vec<Box<dyn Updates>> = vec![d, ct];
    while !(rendering_component.console().window_closed() || game.exit) {
        //root.flush();

        let key = rendering_component.wait_for_keypress();
        println!("Pressed key: {:?}", key);
        match key.code {
            KeyCode::Escape => game.exit = true,
            _ => {}
        }
        update(&mut npcs, &mut c, key, &game);
        render(&mut rendering_component, &npcs, &c);
    }
}

fn render(r: &mut Box<RenderingComponent>, npcs: &Vec<Box<dyn Updates>>, c: &Character) {
    r.before_render_new_frame();
    for i in npcs.iter() {
        i.render(r);
    }
    c.render(r);
    r.after_render_new_frame();
}

fn update(npcs: &mut Vec<Box<dyn Updates>>, c: &mut Character, keypress: tcod::input::Key, g: &Game) {
    c.update(keypress, g);
    for i in npcs.iter_mut() {
        i.update(g);
    }
}
