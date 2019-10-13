use glium::Surface;
use glium_sdl2::DisplayBuild;
use nalgebra::{Rotation3, Vector3};
use sdl2::event::Event;
use std::time::SystemTime;
use std::*;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    color: [f32; 4],
}
glium::implement_vertex!(Vertex, position, normal, color);

impl Vertex {
    fn new(vec: &Vector3<f32>, norm: &Vector3<f32>) -> Vertex {
        Vertex {
            position: [vec.x, vec.y, vec.z],
            normal: [norm.x, norm.y, norm.z],
            color: [1., 1., 1., 1.],
        }
    }

    fn color(mut self, color: [f32; 4]) -> Vertex {
        self.color = color;
        self
    }
}

fn make_rect(
    center: [f32; 3],
    vertex1: [f32; 3],
    color: [f32; 4],
    offset: u32,
) -> (Vec<Vertex>, Vec<u32>) {
    let center = Vector3::from_row_slice(&center);
    let axis = center.normalize();
    let vertex1 = Vector3::from_row_slice(&vertex1) - center;
    let vertexn = |n: usize| {
        Vertex::new(
            &(center + Rotation3::new(axis * -f32::consts::FRAC_PI_2 * n as f32) * vertex1),
            &axis,
        )
        .color(color)
    };

    (
        (0..4).map(vertexn).collect(),
        [0, 1, 2, 2, 3, 0].into_iter().map(|n| n + offset).collect(),
    )
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);
    gl_attr.set_depth_size(24);

    let display = video_subsystem
        .window("OpenGL SDL", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        blend: glium::Blend::alpha_blending(),
        ..Default::default()
    };
    let (shape, indices) = [
        make_rect([0.0, 0.0, 0.5], [0.5, -0.5, 0.5], [1.0, 0.0, 0.0, 0.95], 0),
        make_rect([0.0, 0.5, 0.0], [0.5, 0.5, 0.5], [0.0, 1.0, 0.0, 0.95], 4),
        make_rect(
            [0.0, -0.5, -0.0],
            [-0.5, -0.5, -0.5],
            [0.0, 1.0, 0.0, 0.95],
            8,
        ),
        make_rect([0.5, 0.0, 0.0], [0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 0.95], 12),
        make_rect(
            [-0.5, 0.0, 0.0],
            [-0.5, 0.5, -0.5],
            [0.0, 0.0, 1.0, 0.95],
            16,
        ),
        make_rect(
            [0.0, 0.0, -0.5],
            [-0.5, 0.5, -0.5],
            [1.0, 0.0, 0.0, 0.95],
            20,
        ),
    ]
    .iter_mut()
    .fold(
        (Vec::new(), Vec::new()),
        |mut a: (Vec<Vertex>, Vec<u32>), b| {
            a.0.append(&mut b.0);
            a.1.append(&mut b.1);
            a
        },
    );

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_buffer = glium::index::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    let program = glium::Program::from_source(
        &display,
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
        None,
    )
    .unwrap();

    let mut running = true;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let time = SystemTime::now();

    while running {
        let t = time.elapsed().unwrap().as_millis() as f32 / 1000.0; // precision reduced after 4.6h
        let theta = (0.21 * t * std::f32::consts::PI).sin() * 0.5 * std::f32::consts::PI;
        let sigma = (0.57 * t * std::f32::consts::PI).cos() * 0.5 * std::f32::consts::PI;
        let transform = [
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos() * sigma.cos(), sigma.sin(), 0.0],
            [0.0, -sigma.sin(), sigma.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.3f32],
        ];
        let normal_transform = [
            // same as transform as long as it is only a rotation
            [theta.cos(), theta.sin(), 0.0],
            [-theta.sin(), theta.cos() * sigma.cos(), sigma.sin()],
            [0.0, -sigma.sin(), sigma.cos()],
        ];

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &glium::uniform! {
                    transform : transform,
                    light_color: [1.0f32, 1.0, 1.0],
                    light_position: [1.0f32, 1.0, 1.0],
                    light_ambient: 0.1f32,
                    normal_transform: normal_transform
                },
                &params,
            )
            .unwrap();
        frame.finish().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => running = false,
                _ => (),
            }
        }
    }
}
