use std::collections::HashMap;

use cgmath::{InnerSpace, Matrix3, Rad, vec2, vec3};
use glium::winit::{
    self,
    keyboard::{Key, NamedKey, SmolStr},
};

use crate::ecs::{Components, Entities, Resources, User};

fn check(map: &mut HashMap<winit::keyboard::Key, bool>, key: winit::keyboard::Key) -> bool {
    if !map.contains_key(&key) {
        map.insert(key.clone(), false);
        return false;
    } else {
        return map[&key];
    }
}

const MOVE_SPEED: f32 = 1.6;

pub fn movement_func(
    _: &mut Entities,
    _: &mut Components,
    recources: &mut Resources,
    _: Option<&User>,
) {
    let map = &mut recources.held_keys;
    let cam_pos = recources.thing.cam_pos;
    let cam_rotation = recources.thing.cam_rotation;
    let cam_front = [0.0, 0.0, 1.0];

    let mut cam_pos = vec3(cam_pos[0], cam_pos[1], cam_pos[2]);
    let mut cam_rotation = vec2(cam_rotation[0], cam_rotation[1]);

    let cam_up = vec3(0.0, 1.0, 0.0f32);
    let cam_direction = Matrix3::from_angle_y(Rad {
        0: cam_rotation.x.to_radians(),
    }) * vec3(cam_front[0], cam_front[1], cam_front[2]);

    if check(map, Key::Named(NamedKey::Space)) {
        cam_pos += cam_up * MOVE_SPEED;
    }
    if check(map, Key::Named(NamedKey::Control)) {
        cam_pos -= cam_up * MOVE_SPEED;
    }

    if check(map, Key::Character(SmolStr::new("w"))) {
        cam_pos += cam_direction * MOVE_SPEED;
    }
    if check(map, Key::Character(SmolStr::new("s"))) {
        cam_pos -= cam_direction * MOVE_SPEED;
    }
    if check(map, Key::Character(SmolStr::new("a"))) {
        cam_pos += cam_direction.cross(cam_up).normalize() * MOVE_SPEED;
    }
    if check(map, Key::Character(SmolStr::new("d"))) {
        cam_pos -= cam_direction.cross(cam_up).normalize() * MOVE_SPEED;
    }

    recources.thing.cam_pos = cam_pos.into();
}
