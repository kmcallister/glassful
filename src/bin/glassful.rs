#![feature(io)]
#![deny(warnings)]

use std::old_io as io;

extern crate glassful;

pub fn main() {
    let prog = io::stdin().read_to_end().unwrap();
    let prog = String::from_utf8(prog).unwrap();
    print!("{}", glassful::translate(prog));
}
