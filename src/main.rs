#[macro_use]
extern crate glium;

use glium::SwapBuffersError;
use glium::{glutin, Surface};

struct GlParts<'a> {
    display: &'a glium::Display,
    target: glium::Frame,
    program: &'a glium::Program,
}

impl<'a> GlParts<'a> {
    fn finish(self) -> Result<(), SwapBuffersError> {
        self.target.finish()
    }
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
        get_file_string("./shaders/simple.vert").as_str(),
        get_file_string("./shaders/simple.frag").as_str(),
        None,
    ).unwrap();
    let mut closed = false;
    while !closed {
        let mut gl_parts = GlParts {
            display: &display,
            target: display.draw(),
            program: &program,
        };
        gl_parts.target.clear_color(0.0, 0.0, 1.0, 1.0);
        render(&mut gl_parts);
        gl_parts.finish().unwrap();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => {}
            },
            _ => (),
        });
    }
}

fn render(gl_parts: &mut GlParts) {
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

fn render_tri(verts: Vec<Vertex>, gl_parts: &mut GlParts) {
    let vertex_buffer = glium::VertexBuffer::new(gl_parts.display, &verts).unwrap();
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
