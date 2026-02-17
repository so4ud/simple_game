use cgmath::{InnerSpace, Matrix3, Rad, vec2, vec3};
use glium::winit::{
    event::KeyEvent,
    keyboard::{Key, NamedKey},
};

pub fn handle_key_evnet(
    event: KeyEvent,
    cam_pos: &mut [f32; 3],
    cam_rotation: &mut [f32; 2],
    cam_front: &[f32; 3],
) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = 3.751;
    let mut cam_pos1 = vec3(cam_pos[0], cam_pos[1], cam_pos[2]);
    let mut cam_rotation1 = vec2(cam_rotation[0], cam_rotation[1]);

    let v_cam_up = vec3(0.0, 1.0, 0.0f32);
    let v_cam_direction = Matrix3::from_angle_y(Rad {
        0: cam_rotation1.x.to_radians(),
    }) * vec3(cam_front[0], cam_front[1], cam_front[2]);

    match event.logical_key {
        Key::Named(n) => match n {
            NamedKey::Space => {
                cam_pos1 += v_cam_up * MOVE_SPEED;
            }
            NamedKey::Control => {
                cam_pos1 -= v_cam_up * MOVE_SPEED;
            }
            NamedKey::ArrowRight => {
                cam_rotation1.x += ROTATION_SPEED;
            }
            NamedKey::ArrowLeft => {
                cam_rotation1.x -= ROTATION_SPEED;
            }
            NamedKey::ArrowDown => {
                cam_rotation1.y += ROTATION_SPEED;
            }
            NamedKey::ArrowUp => {
                cam_rotation1.y -= ROTATION_SPEED;
            }
            NamedKey::F12 => {
                panic!("F12 pressed. Process stopped");
            }
            _ => {}
        },
        Key::Character(c) => match c.as_str() {
            "w" => {
                cam_pos1 += v_cam_direction * MOVE_SPEED;
            }
            "s" => {
                cam_pos1 -= v_cam_direction * MOVE_SPEED;
            }
            "a" => {
                cam_pos1 += v_cam_direction.cross(v_cam_up).normalize() * MOVE_SPEED;
            }
            "d" => {
                cam_pos1 -= v_cam_direction.cross(v_cam_up).normalize() * MOVE_SPEED;
            }
            _ => (),
        },
        _ => {
            dbg!();
        }
    }
    // dbg!(&cam_pos);
    let kys: [f32; 3] = cam_pos1.into();
    cam_pos[0] = kys[0];
    cam_pos[1] = kys[1];
    cam_pos[2] = kys[2];
    // dbg!(&cam_pos);
    let kys: [f32; 2] = cam_rotation1.into();
    cam_rotation[0] = kys[0];
    cam_rotation[1] = kys[1];
}
