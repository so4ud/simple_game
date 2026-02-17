use image::{self, io};
use std::{hint, io::Cursor, thread::sleep, time::Duration};

use crate::{
    cube::make_cube, keypress_handler::handle_key_evnet, readraw_handler::handle_redraw_request,
};
use cgmath::{
    self, Matrix, Matrix3, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4, frustum,
    num_traits::Float, perspective, vec4,
};
use glium::{
    self, Surface, Texture2d, VertexBuffer, implement_vertex, index, uniform,
    uniforms::AsUniformValue,
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event::{self, DeviceEvent, KeyEvent},
        keyboard::{Key, NamedKey},
        window,
    },
};
// use crate::cube::make_cube;
mod cube;
mod keypress_handler;
mod readraw_handler;

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
    // we actually gonna have to replace alldat with ui
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("kys >_<")
        .with_inner_size(1280, 720)
        .build(&event_loop);

    // We've changed our shape to a rectangle so the image isn't distorted.
    let (shape, indeces) = make_cube(0.2);

    // let (shape, indecies) = make_cube(0.5);

    let indeces =
        glium::index::IndexBuffer::dynamic(&display, index::PrimitiveType::TrianglesList, &indeces)
            .unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex_shader_src = include_str!("vert.glsl");
    let fragment_shader_src = include_str!("frag.glsl");
    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let image = image::load(
        std::io::Cursor::new(&include_bytes!("defoult_texture.png")[..]),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let raw_image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();

    let mut cam_pos = [0.0, 0.0, 0.5f32];
    let mut cam_direction = [0.0, 0.0, 1.0f32];
    let mut cam_up = [0.0, 1.0, 0.0f32];
    let mut cam_rotation = [0.0, 0.0f32];
    let mut t = 0.0f32;

    let (height, width) = (window.inner_size().height, window.inner_size().width);
    window.set_cursor_visible(false);
    window
        .set_cursor_position(PhysicalPosition::new(width / 2, height / 2)) // relative to te window YEEEEEEEEEEEEEEEEEEEEA
        .unwrap();

    // TODO add the key tracking thingy for smotther movement

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    glium::winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        handle_redraw_request(
                            &display,
                            &mut t,
                            &vertex_buffer,
                            &indeces,
                            &program,
                            &texture,
                            &cam_pos,
                            &cam_direction,
                            &cam_up,
                            &cam_rotation,
                        );
                    }
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    glium::winit::event::WindowEvent::KeyboardInput {
                        device_id: _,
                        event,
                        is_synthetic: _,
                    } => {
                        handle_key_evnet(event, &mut cam_pos, &mut cam_rotation, &cam_direction);
                    }
                    glium::winit::event::WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                    } => {
                        const CAM_ROTATION_SPEED_MULTIPLIER: f32 = 0.1;
                        let (x, y) = (
                            position.x as f32 - (width / 2) as f32,
                            position.y as f32 - (height / 2) as f32,
                        );
                        cam_rotation[0] += x * CAM_ROTATION_SPEED_MULTIPLIER;

                        let temp_y = cam_rotation[1] + y * CAM_ROTATION_SPEED_MULTIPLIER;

                        if temp_y > 89.9 {
                            cam_rotation[1] = 89.9;
                        } else if temp_y < -89.9 {
                            cam_rotation[1] = -89.9;
                        } else {
                            cam_rotation[1] += y * CAM_ROTATION_SPEED_MULTIPLIER;
                        }

                        window
                            .set_cursor_position(PhysicalPosition::new(width / 2, height / 2))
                            .unwrap();
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                    // window.set_decorations(false);
                }
                _ => (),
            }
        })
        .unwrap();
}
