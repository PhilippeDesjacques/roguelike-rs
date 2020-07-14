use crate::util::{Bound, Point};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};
use tcod::RootConsole;
use tcod::input::Key;
use crate::actor::Actor;
use crate::window::{WindowComponent, StatsWindowComponent, InputWindowComponent, MessageWindowComponent, MapWindowComponent};

pub struct Game {
    pub exit: bool,
    pub windows_bounds: Bound,
    pub rendering_component: Box<dyn RenderingComponent + 'static>,
    pub stats_window: Box<dyn WindowComponent>,
    pub input_window: Box<dyn WindowComponent>,
    pub message_window: Box<dyn WindowComponent>,
    pub map_window: Box<dyn WindowComponent>,
    last_key_pressed: Option<Key>,
    character_position: Point,
}

impl Game {
    pub fn new() -> Game {
        //Bound{min: Point{x: 0, y: 0}, max: Point{x: 79, y: 49}}
        let total_bounds = Bound::new(0, 0,  99, 61);
        let stats_bounds = Bound::new(79, 0,  99, 49);
        let input_bounds = Bound::new(0, 50,  99, 52);
        let message_bounds = Bound::new(0, 53,  99, 61);
        let map_bounds = Bound::new(0, 0,  78, 49);
        let root = RootConsole::initializer().size(total_bounds.max.x + 1, total_bounds.max.y + 1).title("libtcod Rust Tutorial").init();
        let rc = Box::new(TcodRenderingComponent::new(root)) as Box<dyn RenderingComponent>;
        let sw = Box::new(StatsWindowComponent::new(stats_bounds)) as Box<dyn WindowComponent>;
        let iw = Box::new(InputWindowComponent::new(input_bounds)) as Box<dyn WindowComponent>;
        let mw = Box::new(MessageWindowComponent::new(message_bounds)) as Box<dyn WindowComponent>;
        let maw = Box::new(MapWindowComponent::new(map_bounds)) as Box<dyn WindowComponent>;
        Game{
            exit: false,
            windows_bounds: total_bounds,
            rendering_component: rc,
            stats_window: sw,
            input_window: iw,
            message_window: mw,
            map_window: maw,
            last_key_pressed: None,
            character_position: Point{x: 0, y: 0}
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, c: &Actor) {
        self.rendering_component.before_render_new_frame();
        self.rendering_component.attach_window(&mut self.stats_window);
        self.rendering_component.attach_window(&mut self.input_window);
        self.rendering_component.attach_window(&mut self.message_window);
        self.rendering_component.attach_window(&mut self.map_window);
        for i in npcs.iter() {
            i.render(&mut self.rendering_component);
        }
        c.render(&mut self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Box<Actor>>, c: &mut Actor) {
        c.update(self);
        self.character_position = c.position.clone();
        for i in npcs.iter_mut() {
            i.update(self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let ks = self.rendering_component.wait_for_keypress();
        self.set_last_key_pressed(Some(ks));
        ks
    }

    pub fn set_last_key_pressed(&mut self, last: Option<Key>) {
        self.last_key_pressed = last;
    }

    pub fn last_key_pressed(&self) -> Option<Key> {
        self.last_key_pressed
    }

    pub fn character_position(&self) -> &Point {
        &self.character_position
    }
}