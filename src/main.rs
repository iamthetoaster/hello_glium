#[macro_use]
extern crate glium;
extern crate image;

mod sphere_gen;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        col: [f32; 4]
    }

    implement_vertex!(Vertex, position, col);

    let vertex1 = Vertex { position: [-0.5, -0.5], col: [0.0, 1.0, 0.0, 1.0] };
    let vertex2 = Vertex { position: [ 0.0,  0.5], col: [1.0, 0.0, 0.0, 1.0] };
    let vertex3 = Vertex { position: [ 0.5, -0.25], col: [0.0, 0.0, 1.0, 1.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec4 col;
        out vec4 colr;
        uniform mat4 matrix;

        void main() {
            colr = col;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec4 colr;
        out vec4 color;

        void main() {
            color = colr;
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut t: f32 = 0.0;
    let mut closed = false;
    while !closed {
        t += 0.0004;
        let uniforms = uniform!{
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [     0.0,     0.0, 1.0, 0.0],
                [     0.0,     0.0, 0.0, 1.0f32]
            ]
        };
        let mut target = display.draw();
        target.clear_color(0.7, 0.7, 1.0, 1.0);


        target.draw(
            &vertex_buffer, &indices, &program, &uniforms, &Default::default()
        ).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}