#![feature(plugin)]
#![feature(std_misc, io)]
#![deny(warnings)]

#[plugin] #[no_link] extern crate glium_macros;
#[plugin] #[no_link] extern crate glassful_macros;

extern crate glutin;
#[macro_use] extern crate glium;

use glium::{DisplayBuild, Program, Surface};
use glium::uniforms::EmptyUniforms;

use std::old_io::timer;
use std::time::Duration;
use std::default::Default;

#[vertex_format]
#[derive(Copy)]
struct Vertex {
    position: [f32; 2],
}

const VERTEX: &'static str = glassful! {
    #![version="110"]

    #[attribute] static position: vec2 = UNINIT;
    #[varying]   static color:    vec3 = UNINIT;

    fn main() {
        gl_Position = vec4(position, 0.0, 1.0);
        color = vec3(0.5*(position + vec2(1.0, 1.0)), 0.0);
    }

    const _work_around_rust_21825: f32 = 0.0;
};

const FRAGMENT: &'static str = glassful! {
    #![version="110"]

    #[varying] static color: vec3 = UNINIT;

    fn main() {
        gl_FragColor = vec4(color, 1.0);
    }

    const _work_around_rust_21825: f32 = 0.0;
};

pub fn main() {
    let dpy = glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&dpy, vec![
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ]);

    let index_buffer = glium::IndexBuffer::new(&dpy,
        glium::index::TrianglesList(vec![0u16, 1, 2, 2, 3, 0]));

    let program = Program::from_source(&dpy, VERTEX, FRAGMENT, None).unwrap();

    loop {
        let mut target = dpy.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program,
            EmptyUniforms, &Default::default()).unwrap();
        target.finish();

        timer::sleep(Duration::milliseconds(20));

        for event in dpy.poll_events() {
            if let glutin::Event::Closed = event {
                return;
            }
        }
    }
}
