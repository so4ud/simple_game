use cgmath::{InnerSpace, Matrix3, Rad, vec2, vec3};
use glium::{
    Display,
    glutin::surface::WindowSurface,
    winit::{
        dpi::{LogicalPosition, PhysicalSize},
        event::{ElementState, KeyEvent},
        keyboard::{Key, NamedKey},
        window::Window,
    },
};

pub fn handle_key_evnet(
    event: KeyEvent,
    cam_pos: &mut [f32; 3],
    cam_rotation: &mut [f32; 2],
    cam_front: &[f32; 3],
    window: &mut Window,
    display: &mut Display<WindowSurface>,
    is_borderles: &mut bool,
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
            NamedKey::Pause => {
                panic!("Pause pressed. Process stopped");
            }
            NamedKey::F11 => match event.state {
                ElementState::Pressed => {
                    dbg!();
                    if *is_borderles == false {
                        window.set_decorations(false);
                        let _ = window.request_inner_size(PhysicalSize::new(1920, 1080));
                        window.set_outer_position(LogicalPosition::new(0.0, 0.0));
                        display.resize((1920, 1080));
                        *is_borderles = !*is_borderles;
                    } else {
                        window.set_decorations(true);
                        let _ = window.request_inner_size(PhysicalSize::new(1280, 720));
                        display.resize((1280, 720));
                        window.set_outer_position(LogicalPosition::new(
                            1920 / 2 - 1280 / 2,
                            1080 / 2 - 720 / 2,
                        ));
                        *is_borderles = !*is_borderles;
                    }
                }
                _ => (),
            },
            _ => {}
        },
        Key::Character(c) => match c.as_str() {
            "w" => {
                cam_pos1 += v_cam_direction * MOVE_SPEED;
                match event.state {
                    // TODO yes you can do dat lets make the thang yahoooo
                    ElementState::Released => {
                        // dbg!("released");
                    }
                    _ => (),
                }
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
