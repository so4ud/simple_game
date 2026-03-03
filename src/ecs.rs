use egui::PaintCallback;
use glium::winit::application::ApplicationHandler;

use crate::User;

/// THE ECS archetechture
/// Im Sold integrate this in the game (and refactor to app.run() if posible)

// also maybe array of u128's if u128 isnt enought
type SIGTYPE = u128;

pub struct Ecs {
    pub entities: Entities,
    pub components: Components,
    pub systems: Systems,
    pub resources: Resources,
}
impl Ecs {
    pub fn new() -> Self {
        let entities = Entities { entities: vec![] };
        let components = Components {};
        let systems = Systems { systems: vec![] };
        let resources = Resources {};

        Self {
            entities,
            components,
            systems,
            resources,
        }
    }
}

// ! finish this
impl ApplicationHandler<User> for Ecs {
    fn window_event(
        &mut self,
        event_loop: &glium::winit::event_loop::ActiveEventLoop,
        window_id: glium::winit::window::WindowId,
        event: glium::winit::event::WindowEvent,
    ) {
    }

    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {}
    fn user_event(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop, event: User) {}
}

struct Entities {
    entities: Vec<Entity>,
}
/// id or signature
struct Entity(SIGTYPE);

struct Components {
    // put components here
    // positions: Vec<Option<Position>>,
}

struct Systems {
    systems: Vec<System>,
}
struct System {}

struct Resources {
    // put resources here
}

pub fn man() {}
