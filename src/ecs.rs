use std::collections::{HashMap, VecDeque};

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
    hendle_cursour,
    keypress_handler::handle_key_evnet,
    redraw_hendler::{self, render_ui},
};
#[derive(Debug)]
pub enum User {
    Update,
    Startup,
}

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
            first: true,
            proxy: event_loop.create_proxy(),
        };
        let held_keys = HashMap::new();

        let thing = Self::innit_thing(&display);

        let entities = Entities { entities: vec![] };
        let components = Components {};
        let systems = Systems { systems: vec![] };
        let resources = Resources {
            event_emiter,
            window,
            held_keys,
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

    fn innit_thing(display: &Display<WindowSurface>) -> Thing {
        let cam_pos: [f32; 3] = [0.0, 0.0, 0.5f32];
        let cam_direction: [f32; 3] = [0.0, 0.0, 1.0f32];
        let cam_up: [f32; 3] = [0.0, 1.0, 0.0f32];
        let cam_rotation: [f32; 2] = [0.0, 0.0f32];
        let is_borderless: bool = false;
        let mouse_mode: bool = false;
        let t: f32 = 0.0f32;

        let (shape, indeces) = make_cube(0.2);

        let indeces: glium::IndexBuffer<u32> = glium::index::IndexBuffer::dynamic(
            display,
            index::PrimitiveType::TrianglesList,
            &indeces,
        )
        .unwrap();
        let vertex_buffer: glium::VertexBuffer<crate::Vertex> =
            glium::VertexBuffer::new(display, &shape).unwrap();

        let vertex_shader_src = include_str!("vert.glsl");
        let fragment_shader_src = include_str!("frag.glsl");
        let program: glium::Program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();
        let vertex_shader_src = include_str!("ui_vert.glsl");
        let fragment_shader_src = include_str!("ui_frag.glsl");
        let ui_program: glium::Program =
            glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        let image = image::load(
            // std::io::Cursor::new(&include_bytes!("defoult_texture.png")[..]),
            std::io::Cursor::new(&include_bytes!("cat.png")[..]),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();
        let raw_image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture: glium::Texture2d = glium::texture::Texture2d::new(display, raw_image).unwrap();

        Thing {
            cam_direction,
            cam_pos,
            cam_rotation,
            cam_up,
            is_borderless,
            mouse_mode,
            t,
            indeces,
            vertex_buffer,
            program,
            ui_program,
            texture,
        }
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

    pub fn add_system(&mut self, system: System) {
        self.systems.systems.push(system);
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
        self.resources.window.set_cursor_visible(false);

        match event {
            glium::winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            glium::winit::event::WindowEvent::CursorMoved {
                device_id,
                position,
            } => hendle_cursour::handle_cursor(self, position),
            // We now need to render everyting in response to a RedrawRequested event due to the animation
            glium::winit::event::WindowEvent::RedrawRequested => {
                let mut target = self.resources.display.draw();

                // indeces: glium::IndexBuffer<u32>,
                // vertex_buffer: glium::VertexBuffer<crate::Vertex>,
                // program: glium::Program,
                // ui_program: glium::Program,
                // texture: glium::Texture2d,

                redraw_hendler::render_scene(
                    &mut target,
                    &mut self.resources.thing.t,
                    &self.resources.thing.vertex_buffer,
                    &self.resources.thing.indeces,
                    &self.resources.thing.program,
                    &self.resources.thing.texture,
                    &self.resources.thing.cam_pos,
                    &self.resources.thing.cam_direction,
                    &self.resources.thing.cam_up,
                    &self.resources.thing.cam_rotation,
                );
                render_ui(
                    &mut self.resources.display,
                    &mut target,
                    &self.resources.thing.ui_program,
                );
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
                handle_key_evnet(
                    event,
                    &mut self.resources.held_keys,
                    &mut self.resources.thing.cam_pos,
                    &mut self.resources.thing.cam_rotation,
                    &self.resources.thing.cam_direction,
                    &mut self.resources.window,
                    &mut self.resources.display,
                    &mut self.resources.thing.is_borderless,
                    &mut self.resources.thing.mouse_mode,
                );
            }

            _ => (),
        }
    }
    fn resumed(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop) {
        self.resources.window.request_redraw();
    }
    fn user_event(&mut self, event_loop: &glium::winit::event_loop::ActiveEventLoop, event: User) {
        self.systems.invoke(
            &mut self.entities,
            &mut self.components,
            &mut self.resources,
            event,
        );
    }
    fn about_to_wait(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        if !self.resources.event_emiter.first {
            self.resources.event_emiter.emmit(User::Update);
        } else {
            self.resources.event_emiter.emmit(User::Startup);
        }
        self.resources.event_emiter.first = false;
        self.resources.window.request_redraw();
    }
}

pub struct Entities {
    pub entities: Vec<Entity>,
}

/// id or signature
pub struct Entity {
    /// always equals to the index
    pub id: u32,
    /// index
    pub parent_id: Option<u32>,
    /// attached components
    pub signature: SIGTYPE,
}

//
pub struct Components {
    // put components here
    // positions: Vec<Option<Position>>,
}

pub struct Systems {
    pub systems: Vec<System>,
}
impl Systems {
    fn invoke(
        &mut self,
        entities: &mut Entities,
        compoents: &mut Components,
        recources: &mut Resources,
        event: User,
    ) {
        match event {
            User::Update => {
                for i in &mut self.systems {
                    match i.invoke_on {
                        User::Update => {
                            i.invoke(entities, compoents, recources, &event);
                        }
                        _ => (),
                    }
                }
            }
            User::Startup => {
                for i in &mut self.systems {
                    match i.invoke_on {
                        User::Startup => {
                            i.invoke(entities, compoents, recources, &event);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

pub struct System {
    pub invoke_on: User,
    pub func: Box<dyn FnMut(&mut Entities, &mut Components, &mut Resources, Option<&User>)>,
}
impl System {
    pub fn invoke(
        &mut self,
        entities: &mut Entities,
        compoents: &mut Components,
        recources: &mut Resources,
        event: &User,
    ) {
        (self.func)(entities, compoents, recources, Some(event))
    }
}

pub struct Resources {
    // put resources here
    pub event_emiter: EventEmiter,
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub held_keys: HashMap<String, bool>,
    pub thing: Thing,
}

pub struct Thing {
    pub cam_pos: [f32; 3],
    pub cam_direction: [f32; 3],
    pub cam_up: [f32; 3],
    pub cam_rotation: [f32; 2],
    pub is_borderless: bool,
    pub mouse_mode: bool,
    pub t: f32,
    //
    pub indeces: glium::IndexBuffer<u32>,
    pub vertex_buffer: glium::VertexBuffer<crate::Vertex>,
    pub program: glium::Program,
    pub ui_program: glium::Program,
    pub texture: glium::Texture2d,
}

pub struct EventEmiter {
    first: bool,
    proxy: EventLoopProxy<User>,
}
impl EventEmiter {
    pub fn emmit(&mut self, event: User) {
        self.proxy
            .send_event(event)
            .expect("event loop no longer existstss");
    }
}
