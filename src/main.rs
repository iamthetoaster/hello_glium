#[macro_use]
extern crate glium;
extern crate image;

mod sphere_gen;
use rand::prelude::*;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        col: [f32; 4]
    }

    implement_vertex!(Vertex, position, col);
    

    let ico = sphere_gen::icosahedron();

    let mut shape: Vec<Vertex> = Vec::new();

    let mut rng = thread_rng();
    for tri in ico {
        let red: f32 = rng.gen();
        let green: f32 = rng.gen();
        let blue: f32 = rng.gen();
        for point in &tri {
            shape.push( Vertex { position: *point, col: [red, green, blue, 0.7] });
        }
    }

    

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec4 col;
        out vec4 colr;
        uniform mat4 translate;
        uniform mat4 scale;
        uniform mat4 xRotation;
        uniform mat4 yRotation;
        uniform mat4 zRotation;
        uniform mat4 perspective;

        void main() {
            gl_Position = translate * yRotation * xRotation * zRotation * scale * vec4(position, 1.0);
            colr = col * ((-gl_Position.z + 1) / 2);
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
        t += 0.0006;

        let mut target = display.draw();
        target.clear_color_and_depth((0.7, 0.7, 1.0, 1.0), 1.0);

        let translate_vector = [0.0, 0.0, 0.0f32];

        let scale_vector = [0.7, 0.7, 0.7f32];

        let rotation_vector = [0.5 * t, t, 0.25 * t];

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
        
            let fov: f32 = std::f32::consts::PI / 6.0;
            let zfar = 2048.0;
            let znear = 0.1;
        
            let f = 1.0 / (fov / 2.0).tan();
        
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let translate = {
            let [x, y, z] = translate_vector;
            [
                [1.0, 0.0, 0.0,   x],
                [0.0, 1.0, 0.0,   y],
                [0.0, 0.0, 1.0,   z],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        let scale = {
            let [x, y, z] = scale_vector;
            [
                [  x, 0.0, 0.0, 0.0],
                [0.0,   y, 0.0, 0.0],
                [0.0, 0.0,   z, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };

        let y_rotation = {
            let ang = rotation_vector[1];
            [
                [ ang.cos(), 0.0, ang.sin(), 0.0],
                [       0.0, 1.0,       0.0, 0.0],
                [-ang.sin(), 0.0, ang.cos(), 0.0],
                [       0.0, 0.0,       0.0, 1.0f32]   
            ]
        };

        let x_rotation = {
            let ang = rotation_vector[0];
            [
                [1.0,       0.0,        0.0, 0.0],
                [0.0, ang.cos(), -ang.sin(), 0.0],
                [0.0, ang.sin(),  ang.cos(), 0.0],
                [0.0,       0.0,        0.0, 1.0f32]
            ]
        };

        let z_rotation = {
            let ang = rotation_vector[2];
            [
                [ang.cos(), -ang.sin(), 0.0, 0.0],
                [ang.sin(),  ang.cos(), 0.0, 0.0],
                [       0.0,       0.0, 1.0, 0.0],
                [       0.0,       0.0, 0.0, 1.0f32]   
            ]
        };

        let uniforms = uniform!{
            perspective: perspective,
            translate: translate,
            scale: scale, 
            yRotation: y_rotation,
            xRotation: x_rotation,
            zRotation: z_rotation
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(
            &vertex_buffer, &indices, &program, &uniforms, &params
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