#![deny(warnings)]

extern crate glassful;

use std::io;
use std::io::Read;

pub fn main() {
    let mut prog = String::new();
    if io::stdin().read_to_string(&mut prog).is_ok() {
        print!("{}", glassful::translate(prog));
    }
}
