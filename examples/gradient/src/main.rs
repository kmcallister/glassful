#![feature(plugin)]

#![plugin(glassful_macros)]

#[macro_use] extern crate glium;

use glium::{DisplayBuild, Program, Surface};
use glium::uniforms::EmptyUniforms;

use std::time::Duration;
use std::thread;
use std::default::Default;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

const VERTEX: &'static str = glassful! {
    #![version="110"]

    #[attribute] static position: vec2 = UNINIT;
    #[varying]   static color:    vec3 = UNINIT;

    fn main() {
        gl_Position = vec4(position, 0.0, 1.0);
        color = vec3(0.5*(position + vec2(1.0, 1.0)), 0.0);
    }
};

const FRAGMENT: &'static str = glassful! {
    #![version="110"]

    #[varying] static color: vec3 = UNINIT;

    fn main() {
        gl_FragColor = vec4(color, 1.0);
    }
};

pub fn main() {
    let dpy = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&dpy, &vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ]).unwrap();

    let index_buffer = glium::IndexBuffer::new(&dpy,
        glium::index::PrimitiveType::TrianglesList, &vec![0u16, 1, 2, 2, 3, 0]).unwrap();

    let program = Program::from_source(&dpy, VERTEX, FRAGMENT, None).unwrap();

    loop {
        let mut target = dpy.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program,
            &EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        thread::sleep(Duration::from_millis(20));

        for event in dpy.poll_events() {
            if let glium::glutin::Event::Closed = event {
                return;
            }
        }
    }
}
