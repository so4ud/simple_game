use image::{self, io};
use std::collections::{HashMap, HashSet};

use crate::ecs::{Ecs, System, User};
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

fn check(map: &mut HashMap<winit::keyboard::Key, bool>, key: winit::keyboard::Key) -> bool {
    if !map.contains_key(&key) {
        map.insert(key.clone(), false);
        return false;
    } else {
        return map[&key];
    }
}

const MOVE_SPEED: f32 = 0.5;

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
        func: Box::new(|_, _, recourses, _| {
            let p = &mut recourses.held_keys;

            if check(p, Key::Named(NamedKey::Space)) {
                let cam_up = vec3(
                    recourses.thing.cam_up[0],
                    recourses.thing.cam_up[1],
                    recourses.thing.cam_up[2],
                );
                let cam_pos = vec3(
                    recourses.thing.cam_pos[0],
                    recourses.thing.cam_pos[1],
                    recourses.thing.cam_pos[2],
                );

                recourses.thing.cam_pos = Into::<[f32; 3]>::into(cam_pos + cam_up * MOVE_SPEED);
            }
        }),
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
