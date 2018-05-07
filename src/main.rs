#[macro_use]
extern crate glium;

use glium::{glutin, Surface};

struct GlParts {
    display: glium::Display,
    target: glium::Frame,
    program: glium::Program,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions(640, 320);
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = glium::Program::from_source(
        &display,
        get_file_string("./pong_fe_gl/shaders/simple.vert").as_str(),
        get_file_string("./pong_fe_gl/shaders/simple.frag").as_str(),
        None,
    ).unwrap();
    let mut closed = false;
    let mut gl_parts = {
        let target = display.draw();
        GlParts {
            display: display,
            target: target,
            program: program,
        }
    };
    while !closed {
        gl_parts.target = gl_parts.display.draw();
        gl_parts.target.clear_color(0.0, 0.0, 1.0, 1.0);
        render(gl_parts);
        gl_parts.target.finish().unwrap();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => {}
            },
            _ => (),
        });
    }
}

fn render(gl_parts: GlParts) {
    render_tri(
        vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
            Vertex {
                position: [0.0, 1.0],
            },
        ],
        gl_parts,
    );
    render_tri(
        vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0],
            },
            Vertex {
                position: [0.0, -1.0],
            },
        ],
        gl_parts,
    );
}

fn render_tri(verts: Vec<Vertex>, gl_parts: GlParts) {
    let vertex_buffer = glium::VertexBuffer::new(&gl_parts.display, &verts).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    gl_parts
        .target
        .draw(
            &vertex_buffer,
            &indices,
            &gl_parts.program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}

fn get_file_string(file_name: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(file_name).unwrap();
    let mut file_src = String::new();
    file.read_to_string(&mut file_src).unwrap();
    file_src
}
