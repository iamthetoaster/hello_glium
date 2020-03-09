#[macro_use]
extern crate glium;
extern crate image;
extern crate noise;

use noise::{NoiseFn, Perlin};

mod obj_tools;
mod types;


use types::TexturedVertex;
use obj_tools::*;   


fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let shape = perlinize(&sphereize(&subdivide(&parse_uv_obj("src/res/icosahedron.obj"), 75)));

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec4 position;
        in vec4 normal;
        in vec3 uv;
        out vec3 transformed_normal;
        out vec4 v_normal;
        uniform mat4 translate;
        uniform mat4 scale;
        uniform mat4 xRotation;
        uniform mat4 yRotation;
        uniform mat4 zRotation;
        uniform mat4 perspective;

        void main() {
            mat4 transform = translate * yRotation * xRotation * zRotation * scale;
            v_normal = normal;
            transformed_normal = transpose(inverse(mat3(transform))) * normal.xyz;
            gl_Position = perspective * transform * position;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 transformed_normal;
        in vec4 v_normal;
        out vec4 color;

        uniform vec3 u_light;

        void main() {

            float brightness = dot(normalize(transformed_normal), normalize(u_light));

            vec3 bright = normalize(normalize(v_normal.xyz) + vec3(1, 1, 1));
            vec3 dark = bright * 0.3;

            color = vec4(mix(dark, bright, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();


    let mut t: f32 = 0.0;
    let mut closed = false;
    while !closed {
        t += 0.005;

        let mut target = display.draw();
        target.clear_color_and_depth((0.7, 0.7, 1.0, 1.0), 1.0);

        let translate_vector = [0.0, 0.0, 2.0f32];

        let scale_vector = [0.7, 0.7, 0.7f32];

        let rotation_vector = [0.5 * t, t, 0.25 * t];

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
        
            let fov: f32 = std::f32::consts::PI / 3.0;
            let zfar = 10.0;
            let znear = 0.1;
        
            let f = 1.0 / (fov / 2.0).tan();
        
            [
                [f * aspect_ratio, 0.0,                            0.0,   0.0],
                [             0.0,   f,                            0.0,   0.0],
                [             0.0, 0.0,      (zfar+znear)/(zfar-znear),   1.0],
                [             0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let translate = {
            let [x, y, z] = translate_vector;
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [  x,   y,   z, 1.0]
            ]
        };

        let scale = {
            let [x, y, z] = scale_vector;
            [
                [  x, 0.0, 0.0, 0.0],
                [0.0,   y, 0.0, 0.0],
                [0.0, 0.0,   z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        };

        let y_rotation = {
            let ang = rotation_vector[1];
            [
                [ ang.cos(), 0.0, ang.sin(), 0.0],
                [       0.0, 1.0,       0.0, 0.0],
                [-ang.sin(), 0.0, ang.cos(), 0.0],
                [       0.0, 0.0,       0.0, 1.0]   
            ]
        };

        let x_rotation = {
            let ang = rotation_vector[0];
            [
                [1.0,       0.0,        0.0, 0.0],
                [0.0, ang.cos(), -ang.sin(), 0.0],
                [0.0, ang.sin(),  ang.cos(), 0.0],
                [0.0,       0.0,        0.0, 1.0]
            ]
        };

        let z_rotation = {
            let ang = rotation_vector[2];
            [
                [ang.cos(), -ang.sin(), 0.0, 0.0],
                [ang.sin(),  ang.cos(), 0.0, 0.0],
                [       0.0,       0.0, 1.0, 0.0],
                [       0.0,       0.0, 0.0, 1.0]   
            ]
        };

        let uniforms = uniform!{
            perspective: perspective,
            translate: translate,
            scale: scale, 
            yRotation: y_rotation,
            xRotation: x_rotation,
            zRotation: z_rotation,
            u_light: [1.0, 1.0, -1.0f32]
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

fn perlinize(model: &Vec<TexturedVertex>) -> Vec<TexturedVertex> {
    let perlin = Perlin::new();
    reset_normals(&model.iter().map(|value| {
        TexturedVertex::new({

            let noiz = noise::Clamp{source: &perlin, bounds: (-1.0, 1.0)}.get([
                value.position[0] as f64,
                value.position[1] as f64,
                value.position[2] as f64,
            ]) as f32;

            let noiz = ((noiz + 1.0) * 0.5) * ((noiz + 1.0) * 0.5) + 0.5;
            [
                value.position[0] * noiz,
                value.position[1] * noiz,
                value.position[2] * noiz,
                1.0f32
            ]
        }, value.normal, value.uv)
    }).collect())
}