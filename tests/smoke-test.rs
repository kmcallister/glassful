#![feature(core)]
#![deny(warnings)]

use std::borrow::ToOwned;

extern crate glassful;

const TEST_PROG: &'static str = r#"
#![version="110"]

static foo: f32 = 3.0;

#[varying] static zed: vec2 = undef;
#[uniform] static prev_frame: sampler2D = undef;
#[uniform] static _p_scale: f32 = undef;

fn complex_to_tex(p: vec2) -> vec2 {
    (0.5/_p_scale)*p + vec2(0.5, 0.5)
}

fn complex_mul(a: vec2, b: vec2) -> vec2 {
    vec2(a.x*b.x - a.y*b.y, a.x*b.y + a.y*b.x)
}

fn in_bounds(p: vec2) -> bool {
    macro_rules! check {
        ($i:ident) => ((p.$i >= -_p_scale) && (p.$i <= _p_scale))
    }

    check!(x) && check!(y)
}

#[uniform] static param: vec2 = undef;

fn main() {
    let color: vec3;
    let oldpoint: vec2 = complex_mul(zed, zed) + 1.6*param;

    if in_bounds(oldpoint) {
        let oldcoord: vec2 = complex_to_tex(oldpoint);
        color = 1.8*texture2D(prev_frame, oldcoord).brg;
    } else {
        color = vec3(1.0, 0.0, 0.0);
    }

    gl_FragColor = vec4(color, 1.0);
}
"#;

#[test]
fn smoke_test() {
    // Just make sure we can translate without crashing.
    glassful::translate(TEST_PROG.to_owned());
}
