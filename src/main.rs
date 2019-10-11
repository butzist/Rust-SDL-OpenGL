use glium::Surface;
use glium_sdl2::DisplayBuild;
use sdl2::event::Event;
use std::time::SystemTime;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
glium::implement_vertex!(Vertex, position, color);

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: [x, y, z],
            color: [0., 0., 0.],
        }
    }

    fn color(mut self, r: f32, g: f32, b: f32) -> Vertex {
        self.color = [r, g, b];
        self
    }

    fn color_vec(mut self, color: [f32; 3]) -> Vertex {
        self.color = color;
        self
    }
}

/*fn make_rect(center: [f32; 3]) -> Vec<Vertex> {
    let colors = vec![
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.8, 0.0],
    ];

    let direction = center
        .iter()
        .fold(0.0, |a, &b| if a == 0.0 { b } else { a });
    let mut dirs = vec![direction, direction].into_iter();
    let orth = [
        if center[0] == 0.0 {
            dirs.next().unwrap()
        } else {
            0.0
        },
        if center[1] == 0.0 {
            dirs.next().unwrap()
        } else {
            0.0
        },
        if center[2] == 0.0 {
            dirs.next().unwrap()
        } else {
            0.0
        },
    ];

    vec![Vertex::new(-0.5, -0.5, 0.0).color_vec(colors[0])]
}*/

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem
        .window("OpenGL SDL", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    let shape = vec![
        Vertex::new(-0.5, -0.5, -0.5).color(1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5).color(0.0, 1.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5).color(0.0, 0.0, 1.0),
        Vertex::new(0.5, 0.5, -0.5).color(1.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5).color(1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5).color(0.0, 1.0, 0.0),
        Vertex::new(0.5, -0.5, 0.5).color(0.0, 0.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5).color(1.0, 1.0, 0.0),
        Vertex::new(-0.5, -0.5, -0.5).color(1.0, 0.0, 0.0),
        Vertex::new(-0.5, -0.5, 0.5).color(0.0, 1.0, 0.0),
        Vertex::new(0.5, -0.5, -0.5).color(0.0, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.5).color(1.0, 1.0, 0.0),
        Vertex::new(-0.5, 0.5, -0.5).color(1.0, 0.0, 0.0),
        Vertex::new(-0.5, 0.5, 0.5).color(0.0, 1.0, 0.0),
        Vertex::new(0.5, 0.5, -0.5).color(0.0, 0.0, 1.0),
        Vertex::new(0.5, 0.5, 0.5).color(1.0, 1.0, 0.0),
    ];

    let indices = vec![
        0u32, 1, 2, 1, 2, 3, 4, 5, 6, 5, 6, 7, 8, 9, 10, 9, 10, 11, 12, 13, 14, 13, 14, 15,
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    //let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
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
        let theta = (0.1 * t) % 1.0 * 2. * std::f32::consts::PI;
        let sigma = (0.4 * t) % 1.0 * 2. * std::f32::consts::PI + 2.;
        let transform = [
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos() * sigma.cos(), sigma.sin(), 0.0],
            [0.0, -sigma.sin(), sigma.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &glium::uniform! {transform : transform},
                &Default::default(),
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
