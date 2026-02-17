use cgmath::{Deg, Matrix, Matrix4, Point3, Rad, Vector3, perspective, point3};
use cgmath::{Matrix3, vec3};
use glium::{Display, IndexBuffer, Program, VertexBuffer, glutin::surface::WindowSurface, uniform};

use crate::Texture2d;
use crate::Vertex;
use glium::Surface;

pub fn handle_redraw_request(
    display: &Display<WindowSurface>,
    t: &mut f32,
    vertex_buffer: &VertexBuffer<Vertex>,
    indecies: &IndexBuffer<u32>,
    program: &Program,
    texture: &Texture2d,
    cam_pos: &[f32; 3],
    cam_direction: &[f32; 3],
    cam_up: &[f32; 3],
    cam_rotation: &[f32; 2],
) {
    let mut target = display.draw();
    *t += 0.5;
    let cam_pos = Point3::new(cam_pos[0], cam_pos[1], cam_pos[2]);

    let cam_direction = Matrix3::from_angle_y(Rad {
        0: cam_rotation[0].to_radians(),
    }) * Matrix3::from_angle_x(Rad {
        0: cam_rotation[1].to_radians(),
    }) * vec3(cam_direction[0], cam_direction[1], cam_direction[2]);

    let cam_up = Point3::new(cam_up[0], cam_up[1], cam_up[2]);

    let center = point3(cam_pos.x, cam_pos.y, cam_pos.z) - cam_direction;
    let center: (f32, f32, f32) = center.into();
    let center: Point3<f32> = center.into();

    let rot = cgmath::Matrix4::from_angle_y(Rad {
        0: (*t % 360.0).to_radians(),
    });

    let m: Matrix4<f32> = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.4, 0.001f32],
    ]
    .into();

    let m: [[f32; 4]; 4] = (m * rot).into();
    let v: [[f32; 4]; 4] =
        cgmath::Matrix4::look_at_rh(cam_pos, center, Vector3::new(0.0, 1.0, 0.0f32)).into();

    let p: [[f32; 4]; 4] = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 90.0f32.to_radians();
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0f32],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    };
    //    dbg!(&p);

    let (width, height) = target.get_dimensions();
    //    let p: [[f32;4];4] = cgmath::perspective(Deg(90.0), (width/height)as f32, 0.1, 1024.0).into();
    //    dbg!(p);
    //    panic!();
    //    dbg!(&v);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let uniforms = uniform! {
        m: m,
        p: p,
        v: v,
        text: texture,

    };
    target.clear_color_and_depth((0.15, 0.15, 0.15, 1.0), 1.0);

    target
        .draw(vertex_buffer, indecies, program, &uniforms, &params)
        .unwrap();

    let rot = cgmath::Matrix4::from_angle_x(Rad {
        0: (*t / 5.0 % 360.0).to_radians(),
    });

    let m: Matrix4<f32> = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.3, 0.4, 0.001f32],
    ]
    .into();

    let m: [[f32; 4]; 4] = (m * rot).into();
    let v: [[f32; 4]; 4] =
        cgmath::Matrix4::look_at_rh(cam_pos, center, Vector3::new(0.0, 1.0, 0.0f32)).into();

    let p = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 90.0f32.to_radians();
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0f32],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    };
    let uniforms = uniform! {
        m: m,
        p: p,
        v: v,
        text: texture,

    };
    //    target.clear_color_and_depth((0.15, 0.15, 0.15, 1.0), 1.0);

    target
        .draw(vertex_buffer, indecies, program, &uniforms, &params)
        .unwrap();

    let rot = cgmath::Matrix4::from_angle_z(Rad {
        0: (*t / 10.0 % 360.0).to_radians(),
    });

    let m: Matrix4<f32> = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.3, 0.0, 0.4, 0.001f32],
    ]
    .into();

    let m: [[f32; 4]; 4] = (m * rot).into();
    let v: [[f32; 4]; 4] =
        cgmath::Matrix4::look_at_rh(cam_pos, center, Vector3::new(0.0, 1.0, 0.0f32)).into();

    let p = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 90.0f32.to_radians();
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0f32],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    };
    let uniforms = uniform! {
        m: m,
        p: p,
        v: v,
        text: texture,

    };
    //    target.clear_color_and_depth((0.15, 0.15, 0.15, 1.0), 1.0);

    target
        .draw(vertex_buffer, indecies, program, &uniforms, &params)
        .unwrap();

    target.finish().unwrap();
}
