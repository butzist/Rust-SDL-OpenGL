use glium::Surface;
use glium_sdl2::DisplayBuild;
use sdl2::event::Event;
use std::time::SystemTime;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem
        .window("OpenGL SDL", 800, 600)
        .resizable()
        .build_glium()
        .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3],
    }
    glium::implement_vertex!(Vertex, position, color);

    impl Vertex {
        fn new(x: f32, y: f32) -> Vertex {
            Vertex {
                position: [x, y],
                color: [0., 0., 0.],
            }
        }

        fn color(mut self, r: f32, g: f32, b: f32) -> Vertex {
            self.color = [r, g, b];
            self
        }
    }

    let shape = vec![
        Vertex::new(-0.5, -0.5).color(1.0, 0.0, 0.0),
        Vertex::new(0.0, 0.5).color(0.0, 1.0, 0.0),
        Vertex::new(0.5, -0.25).color(0.0, 0.0, 1.0),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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
        let t = time.elapsed().unwrap().as_millis() as f32 / 1000.0;
        let mut frame = display.draw();
        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        frame
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniform! {theta : 0.1 * t * 2. * std::f32::consts::PI},
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
