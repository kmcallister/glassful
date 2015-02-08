# Rust-like syntax for GLSL

[![Build Status](https://travis-ci.org/kmcallister/glassful.svg?branch=master)](https://travis-ci.org/kmcallister/glassful)

glassful translates a small subset of Rust to [OpenGL Shading Language][].

Besides one's personal preferences regarding Rust-like vs. C-like syntax, this
has a few specific advantages for the Rust programmer:

* Syntax is checked at Rust compile time, with friendly rustc-style errors
* There's less cognitive overhead switching between CPU and GPU code
* Shaders can use Rust macros!
* Shaders embedded in a Rust program have syntax highlighting

The library is still in a *very* early stage! Many improvements are possible.
See the [issue tracker][] and don't hesitate to send pull requests :)

[OpenGL Shading Language]: https://www.opengl.org/documentation/glsl/
[issue tracker]: https://github.com/kmcallister/glassful/issues

## Usage

There are three ways to invoke the translator.  The language syntax is exactly
the same in all three cases.

### As a macro

```rust
#[plugin] #[no_link] extern crate glassful_macros;

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

let program = glium::Program::from_source(&display, VERTEX, FRAGMENT, None);
```

See `examples/gradient/` for a full glium/glutin example.

### As an external program

```
$ ./target/glassful < shader.glassful > shader.glsl
```

### As an ordinary library

```rust
extern crate glassful;

pub fn main() {
    let prog = io::stdin().read_to_end().unwrap();
    let prog = String::from_utf8(prog).unwrap();
    print!("{}", glassful::translate(prog));
}
```
