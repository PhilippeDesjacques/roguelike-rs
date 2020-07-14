use crate::util::{Point, Bound};
use crate::game::Game;
use crate::movement::{MovementComponent, InputMovementComponent, RandomMovementComponent, AggroMovementComponent};
use crate::rendering::RenderingComponent;

pub struct Actor {
    pub position: Point,
    pub display_char: char,
    pub movement_component: Box<dyn MovementComponent>,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<dyn MovementComponent>) -> Actor {
        Actor { position: Point { x, y }, display_char: dc, movement_component: mc }
    }

    pub fn update(&mut self, g: &Game) {
        self.movement_component.set_key_pressed(g.last_key_pressed());
        self.position = self.movement_component.update(self.position.clone(), g);
    }

    pub fn render(&self, rendering_component: &mut Box<dyn RenderingComponent>) {
        rendering_component.render_object(&self.position, self.display_char);
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc = Box::new(RandomMovementComponent::new(bound)) as Box<dyn MovementComponent>;
        Actor::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc = Box::new(RandomMovementComponent::new(bound)) as Box<dyn MovementComponent>;
        Actor::new(x, y, 'c', mc)
    }

    pub fn heroine(x: i32, y: i32, bound: Bound) -> Actor {
        let mc = Box::new(InputMovementComponent::new(bound)) as Box<dyn MovementComponent>;
        Actor::new(x, y, '@', mc)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor {
        let mc = Box::new(AggroMovementComponent::new(bound)) as Box<dyn MovementComponent>;
        Actor::new(x, y, 'K', mc)
    }
}