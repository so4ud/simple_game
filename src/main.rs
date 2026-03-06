use image::{self, io};
use std::collections::{HashMap, HashSet};

use crate::{
    ecs::{Ecs, System, User},
    movement_system::movement_func,
};
use cgmath::{self, vec3};
use glium::{
    self, Texture2d, implement_vertex,
    winit::{
        self,
        keyboard::{Key, NamedKey, SmolStr},
    },
};

mod ecs;

// use crate::cube::make_cube;
mod cube;
mod hendle_cursour;
mod keypress_handler;
mod redraw_hendler;

mod movement_system;
#[derive(Debug, Copy, Clone)]
struct UiVertex {
    position: [f32; 3],
    color: [f32; 3],
    uv: [f32; 2],
}
implement_vertex!(UiVertex, position, color, uv);
impl UiVertex {
    fn new(position: [f32; 3], color: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position,
            color,
            uv,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}
implement_vertex!(Vertex, position, normal, uv);
impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position,
            normal,
            uv,
        }
    }
}

/// <h1 style="color:red"> SHUT THE FUCK UUUUUUP </h1>
fn main() {
    let (mut ecs, event_loop) = Ecs::new();
    let hello_system = System {
        invoke_on: User::Update,
        func: Box::new(|_, _, recourses, _| {
            recourses.thing.t += 0.1;
        }),
    };
    let movement_system = System {
        invoke_on: User::Update,
        func: Box::new(movement_func),
    };
    let start_system = System {
        invoke_on: User::Startup,
        func: Box::new(|_, _, _, _| {
            println!("were on bebey!!!!!!!!!!");
        }),
    };
    ecs.add_system(start_system);
    ecs.add_system(movement_system);
    ecs.add_system(hello_system);
    event_loop.run_app(&mut ecs).unwrap();
}
