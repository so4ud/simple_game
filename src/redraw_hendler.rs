use cgmath::{
    Deg, Matrix, Matrix4, Point3, Rad, Vector3, VectorSpace, ortho, perspective, point3, vec2,
};
use cgmath::{Matrix3, vec3};
use glium::{Display, IndexBuffer, Program, VertexBuffer, glutin::surface::WindowSurface, uniform};

use crate::Texture2d;
use crate::{UiVertex, Vertex};
use glium::{Frame, Surface};

pub fn render_ui(display: &mut Display<WindowSurface>, target: &mut Frame, program: &Program) {
    let (width, height) = display.get_framebuffer_dimensions();
    let half_w = width as f32 / 2.0;
    let half_h = height as f32 / 2.0;
    let ortho: [[f32; 4]; 4] = ortho(-half_w, half_w, -half_h, half_h, -0.9, 0.9).into();

    let crosshair_vertecies: [UiVertex; 8] = [
        UiVertex::new([-3.01, -10.03, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]), // 0
        UiVertex::new([-3.01, 10.03, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),  // 1
        UiVertex::new([3.01, 10.03, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),   // 2
        UiVertex::new([3.01, -10.03, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),  // 3
        UiVertex::new([-10.03, -3.01, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]), // 4
        UiVertex::new([-10.03, 3.01, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),  // 5
        UiVertex::new([10.03, 3.01, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),   // 6
        UiVertex::new([10.03, -3.01, 0.00], [0.15, 1.0, 0.15], [0.0, 0.0]),  // 7
    ];
    let indecies: [u32; 12] = [0, 3, 1, 1, 3, 2, 4, 5, 7, 5, 7, 6];

    let vertex_buffer = VertexBuffer::dynamic(display, &crosshair_vertecies).unwrap();
    let indecies = IndexBuffer::dynamic(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &indecies,
    )
    .unwrap();

    let uniforms = uniform! {
        mat: ortho
    };
    // target.clear_color_and_depth((0.15, 0.15, 0.15, 1.0), 1.0);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::Overwrite,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    target
        .draw(&vertex_buffer, &indecies, &program, &uniforms, &params)
        .unwrap();
}

pub fn render_scene(
    target: &mut Frame,
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
        cgmath::Matrix4::look_at_rh(cam_pos, center, vec3(cam_up.x, cam_up.y, cam_up.z)).into();

    let p: [[f32; 4]; 4] = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 90.0f32.to_radians();
        let zfar = 1024.0 * 10.0;
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

    let v1 = vec2(-0.3, 0.3f32);
    let v2 = vec2(0.13, 0.3f32);
    let v3 = vec2(0.13, -0.3f32);
    let v4 = v1.lerp(v2, ((*t / 100.0).sin() + 1.0) / 2.0);
    let v5 = v2.lerp(v3, ((*t / 100.0).sin() + 1.0) / 2.0);
    let v6 = v4.lerp(v5, ((*t / 100.0).sin() + 1.0) / 2.0);
    // dbg!(((*t / 360.0).sin() + 1.0) / 2.0);

    let m: Matrix4<f32> = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [v6.y, v6.x, 0.4, 0.001f32],
    ]
    .into();

    let m: [[f32; 4]; 4] = (m * rot).into();
    let v: [[f32; 4]; 4] =
        cgmath::Matrix4::look_at_rh(cam_pos, center, Vector3::new(0.0, 1.0, 0.0f32)).into();

    let p = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 90.0f32.to_radians();
        let zfar = 1024.0 * 10.0;
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
        let zfar = 1024.0 * 10.0;
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
}
