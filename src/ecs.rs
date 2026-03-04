use std::collections::VecDeque;

use egui::PaintCallback;
use glium::{
    Display,
    glutin::{api::egl::display, surface::WindowSurface},
    index,
    winit::{
        application::ApplicationHandler,
        dpi::PhysicalPosition,
        event_loop::{self, EventLoop, EventLoopProxy},
        window::Window,
    },
};

use crate::{
    cube::make_cube,
    redraw_hendler::{self, render_ui},
};
#[derive(Debug)]
pub enum User {}

/// THE ECS archetechture
/// Im Sold integrate this in the game (and refactor to app.run() if posible)

// also maybe array of u128's if u128 isnt enought
pub type SIGTYPE = u128;

// struct SIGTYPE {
//     inner: [u128; 4]
// }

pub struct Ecs {
    pub entities: Entities,
    pub components: Components,
    pub systems: Systems,
    pub resources: Resources,
}
impl Ecs {
    pub fn new() -> (Self, EventLoop<User>) {
        let (event_loop, window, display) = Self::init_winnit();
        let event_emiter = EventEmiter {
            proxy: event_loop.create_proxy(),
        };
        // let event_loop = Some(event_loop);
        let cam_pos: [f32; 3] = [0.0, 0.0, 0.5f32];
        let cam_direction: [f32; 3] = [0.0, 0.0, 1.0f32];
        let cam_up: [f32; 3] = [0.0, 1.0, 0.0f32];
        let cam_rotation: [f32; 2] = [0.0, 0.0f32];
        let is_borderless: bool = false;
        let mouse_mode: bool = false;
        let t: f32 = 0.0f32;

        let thing = Thing {
            cam_pos,
            cam_direction,
            cam_up,
            cam_rotation,
            is_borderless,
            mouse_mode,
            t,
        };

        let entities = Entities { entities: vec![] };
        let components = Components {};
        let systems = Systems { systems: vec![] };
        let resources = Resources {
            event_emiter,
            window,
            display,
            thing,
        };

        (
            Self {
                entities,
                components,
                systems,
                resources,
            },
            event_loop,
        )
    }

    fn init_winnit() -> (
        glium::winit::event_loop::EventLoop<User>,
        Window,
        Display<WindowSurface>,
    ) {
        let event_loop = glium::winit::event_loop::EventLoop::<User>::with_user_event()
            .build()
            .expect("event loop building");

        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("kys >_<")
            .with_inner_size(1280, 720)
            .build(&event_loop);

        return (event_loop, window, display);
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
        match event {
            glium::winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            // We now need to render everyting in response to a RedrawRequested event due to the animation
            glium::winit::event::WindowEvent::RedrawRequested => {
                let mut target = self.resources.display.draw();

                let (shape, indeces) = make_cube(0.2);

                let indeces = glium::index::IndexBuffer::dynamic(
                    &self.resources.display,
                    index::PrimitiveType::TrianglesList,
                    &indeces,
                )
                .unwrap();
                let vertex_buffer =
                    glium::VertexBuffer::new(&self.resources.display, &shape).unwrap();

                let vertex_shader_src = include_str!("vert.glsl");
                let fragment_shader_src = include_str!("frag.glsl");
                let program = glium::Program::from_source(
                    &self.resources.display,
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                )
                .unwrap();
                let vertex_shader_src = include_str!("ui_vert.glsl");
                let fragment_shader_src = include_str!("ui_frag.glsl");
                let ui_program = glium::Program::from_source(
                    &self.resources.display,
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                )
                .unwrap();

                let image = image::load(
                    // std::io::Cursor::new(&include_bytes!("defoult_texture.png")[..]),
                    std::io::Cursor::new(&include_bytes!("cat.png")[..]),
                    image::ImageFormat::Png,
                )
                .unwrap()
                .to_rgba8();

                let image_dimensions = image.dimensions();
                let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &image.into_raw(),
                    image_dimensions,
                );

                let texture = glium::texture::Texture2d::new(&display, raw_image).unwrap();

                redraw_hendler::render_scene(
                    &mut target,
                    &mut self.resources.thing.t,
                    &vertex_buffer,
                    &indeces,
                    &program,
                    &texture,
                    &self.resources.thing.cam_pos,
                    &self.resources.thing.cam_direction,
                    &self.resources.thing.cam_up,
                    &self.resources.thing.cam_rotation,
                );
                render_ui(&mut self.resources.display, &mut target, &ui_program);
                target.finish().unwrap();
            }
            glium::winit::event::WindowEvent::Resized(window_size) => {
                self.resources.display.resize(window_size.into());
            }
            glium::winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                // maybe turn this int olike render 3d scene or someshi
                // handle_key_evnet(
                //     event,
                //     &mut held_keys,
                //     &mut cam_pos,
                //     &mut cam_rotation,
                //     &cam_direction,
                //     &mut self.resources.window,
                //     &mut self.resources.display,
                //     &mut is_borderless,
                //     &mut mouse_mode,
                // );
            }
            glium::winit::event::WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                /*
                // dbg!(mouse_mode);
                // if mouse_mode == false {
                //     const CAM_ROTATION_SPEED_MULTIPLIER: f32 = 0.1;
                //     let (x, y) = (
                //         position.x as f32 - (width / 2) as f32,
                //         position.y as f32 - (height / 2) as f32,
                //     );
                //     cam_rotation[0] += x * CAM_ROTATION_SPEED_MULTIPLIER;

                //     let temp_y = cam_rotation[1] + y * CAM_ROTATION_SPEED_MULTIPLIER;

                //     if temp_y > 89.9 {
                //         cam_rotation[1] = 89.9;
                //     } else if temp_y < -89.9 {
                //         cam_rotation[1] = -89.9;
                //     } else {
                //         cam_rotation[1] += y * CAM_ROTATION_SPEED_MULTIPLIER;
                //     }

                //     self.resources
                //         .window
                //         .set_cursor_position(PhysicalPosition::new(width / 2, height / 2))
                //         .unwrap();
                // } */
            }

            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {}
    fn user_event(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop, event: User) {}
}

pub struct Entities {
    pub entities: Vec<Entity>,
}

/// id or signature
pub struct Entity {
    /// index
    parent: Option<u32>,
    /// attached components
    signature: SIGTYPE,
}

pub struct Components {
    // put components here
    // positions: Vec<Option<Position>>,
}

pub struct Systems {
    pub systems: Vec<System>,
}
pub struct System {
    invoke_on: User,
    func: Box<dyn FnMut(&mut Components, &mut Resources, Option<User>)>,
    /// maybe
    query: SIGTYPE,
    mut_query: SIGTYPE,
}

pub struct Resources {
    // put resources here
    event_emiter: EventEmiter,
    window: Window,
    display: Display<WindowSurface>,
    thing: Thing,
}

struct Thing {
    cam_pos: [f32; 3],
    cam_direction: [f32; 3],
    cam_up: [f32; 3],
    cam_rotation: [f32; 2],
    is_borderless: bool,
    mouse_mode: bool,
    t: f32,
}

pub struct EventEmiter {
    proxy: EventLoopProxy<User>,
}
impl EventEmiter {
    pub fn emmit(&mut self, event: User) {
        self.proxy.send_event(event);
    }
}

pub fn man() {}
