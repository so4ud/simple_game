use image::{self, io};
use std::{
    collections::{HashMap, HashSet},
    hint,
    io::Cursor,
    ops::Index,
    thread::sleep,
    time::Duration,
};

use crate::{
    cube::make_cube,
    ecs::{Ecs, User},
    keypress_handler::handle_key_evnet,
    redraw_hendler::render_ui,
};
use cgmath::{
    self, Matrix, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4, frustum,
    num_traits::Float, ortho, perspective, vec4,
};
use glium::{
    self, Surface, Texture2d, VertexBuffer, implement_uniform_block, implement_vertex, index,
    uniform,
    uniforms::AsUniformValue,
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event::{self, DeviceEvent, KeyEvent},
        event_loop,
        keyboard::{Key, NamedKey},
        platform::pump_events::EventLoopExtPumpEvents,
        window,
    },
};

mod ecs;

// use crate::cube::make_cube;
mod cube;
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

#[derive(Debug)]
struct App {}

// add app class and also user events work

fn main() {
    let (mut ecs, event_loop) = Ecs::new();

    let mut held_keys = HashMap::<&str, bool>::new();
    {
        held_keys.insert("space", false);
    }

    let mut cam_pos: [f32; 3] = [0.0, 0.0, 0.5f32];
    let mut cam_direction: [f32; 3] = [0.0, 0.0, 1.0f32];
    let mut cam_up: [f32; 3] = [0.0, 1.0, 0.0f32];
    let mut cam_rotation: [f32; 2] = [0.0, 0.0f32];
    let mut is_borderless: bool = false;
    let mut mouse_mode: bool = false;
    let mut t: f32 = 0.0f32;

    // ! let (height, width) = (window.inner_size().height, window.inner_size().width);
    // ! window.set_cursor_visible(false);
    // ! window
    // !     .set_cursor_position(PhysicalPosition::new(width / 2, height / 2)) // relative to te window YEEEEEEEEEEEEEEEEEEEEA
    // !     .unwrap();

    // if true {
    event_loop.run_app(&mut ecs).unwrap();
    // } else {
    //     // TODO add the key tracking thingy for smotther movement
    //     dbg!("sex");
    //     #[allow(deprecated)]
    //     event_loop
    //         .run(move |ev, window_target| {
    //             // window_target.exit();
    //             // yaaaa it works
    //             if !held_keys.contains_key("space") {
    //                 held_keys.insert("space", false);
    //             }
    //             if held_keys["space"] == true {
    //                 cam_pos[1] += 1.0 * 0.3;
    //             }
    //             match ev {
    //                 glium::winit::event::Event::WindowEvent { event, .. } => match event {
    //                     glium::winit::event::WindowEvent::CloseRequested => {
    //                         window_target.exit();
    //                     }
    //                     // We now need to render everyting in response to a RedrawRequested event due to the animation
    //                     glium::winit::event::WindowEvent::RedrawRequested => {
    //                         let mut target = display.draw();

    //                         redraw_hendler::render_scene(
    //                             &mut target,
    //                             &mut t,
    //                             &vertex_buffer,
    //                             &indeces,
    //                             &program,
    //                             &texture,
    //                             &cam_pos,
    //                             &cam_direction,
    //                             &cam_up,
    //                             &cam_rotation,
    //                         );
    //                         render_ui(&mut display, &mut target, &ui_program);
    //                         target.finish().unwrap();
    //                     }
    //                     glium::winit::event::WindowEvent::Resized(window_size) => {
    //                         display.resize(window_size.into());
    //                     }
    //                     glium::winit::event::WindowEvent::KeyboardInput {
    //                         device_id: _,
    //                         event,
    //                         is_synthetic: _,
    //                     } => {
    //                         // maybe turn this int olike render 3d scene or someshi
    //                         handle_key_evnet(
    //                             event,
    //                             &mut held_keys,
    //                             &mut cam_pos,
    //                             &mut cam_rotation,
    //                             &cam_direction,
    //                             &mut window,
    //                             &mut display,
    //                             &mut is_borderless,
    //                             &mut mouse_mode,
    //                         );
    //                     }
    //                     glium::winit::event::WindowEvent::CursorMoved {
    //                         device_id: _,
    //                         position,
    //                     } => {
    //                         // dbg!(mouse_mode);
    //                         if mouse_mode == false {
    //                             const CAM_ROTATION_SPEED_MULTIPLIER: f32 = 0.1;
    //                             let (x, y) = (
    //                                 position.x as f32 - (width / 2) as f32,
    //                                 position.y as f32 - (height / 2) as f32,
    //                             );
    //                             cam_rotation[0] += x * CAM_ROTATION_SPEED_MULTIPLIER;

    //                             let temp_y = cam_rotation[1] + y * CAM_ROTATION_SPEED_MULTIPLIER;

    //                             if temp_y > 89.9 {
    //                                 cam_rotation[1] = 89.9;
    //                             } else if temp_y < -89.9 {
    //                                 cam_rotation[1] = -89.9;
    //                             } else {
    //                                 cam_rotation[1] += y * CAM_ROTATION_SPEED_MULTIPLIER;
    //                             }

    //                             window
    //                                 .set_cursor_position(PhysicalPosition::new(
    //                                     width / 2,
    //                                     height / 2,
    //                                 ))
    //                                 .unwrap();
    //                         }
    //                     }

    //                     _ => (),
    //                 },
    //                 event::Event::UserEvent(sex) => {
    //                     dbg!(sex);
    //                 }

    //                 // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
    //                 // For applications that only change due to user input you could remove this handler.
    //                 glium::winit::event::Event::AboutToWait => {
    //                     window.request_redraw();
    //                     // window.set_decorations(false);
    //                 }
    //                 _ => (),
    //             }
    //         })
    //         .unwrap();
    // }
}
