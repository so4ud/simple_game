use glium::winit::dpi::PhysicalPosition;

use crate::ecs::Ecs;

pub fn handle_cursor(ecs: &mut Ecs, position: PhysicalPosition<f64>) {
    let size = ecs.resources.window.inner_size();
    let (width, height) = (size.width, size.height);

    if ecs.resources.thing.mouse_mode == false {
        const CAM_ROTATION_SPEED_MULTIPLIER: f32 = 0.1;
        let (x, y) = (
            position.x as f32 - (width / 2) as f32,
            position.y as f32 - (height / 2) as f32,
        );
        ecs.resources.thing.cam_rotation[0] += x * CAM_ROTATION_SPEED_MULTIPLIER;

        let temp_y = ecs.resources.thing.cam_rotation[1] + y * CAM_ROTATION_SPEED_MULTIPLIER;

        if temp_y > 89.9 {
            ecs.resources.thing.cam_rotation[1] = 89.9;
        } else if temp_y < -89.9 {
            ecs.resources.thing.cam_rotation[1] = -89.9;
        } else {
            ecs.resources.thing.cam_rotation[1] += y * CAM_ROTATION_SPEED_MULTIPLIER;
        }

        ecs.resources
            .window
            .set_cursor_position(PhysicalPosition::new(width / 2, height / 2))
            .unwrap();
    }
}
