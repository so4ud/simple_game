use image::{self, io};
use std::collections::{HashMap, HashSet};

use crate::ecs::{Ecs, System, User};
use cgmath::{self};
use glium::{self, Texture2d, implement_vertex};

mod ecs;

// use crate::cube::make_cube;
mod cube;
mod hendle_cursour;
mod keypress_handler;
mod redraw_hendler;
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

fn main() {
    let (mut ecs, event_loop) = Ecs::new();
    let hello_system = System {
        invoke_on: User::Update,
        func: Box::new(|_, _, recourses, _| {
            recourses.thing.t += 0.1;
        }),
    };
    let start_system = System {
        invoke_on: User::Startup,
        func: Box::new(|_, _, _, _| {
            println!("were on bebey!!!!!!!!!!");
        }),
    };
    ecs.add_system(start_system);
    ecs.add_system(hello_system);
    event_loop.run_app(&mut ecs).unwrap();
}
