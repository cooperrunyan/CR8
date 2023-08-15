#![feature(fn_traits)]

mod cr8;
mod device;
mod exec;

use std::{env, fs};

use cr8::CR8;

fn main() {
    let args: Vec<_> = env::args().collect();
    let path = args[1];

    let instructions = fs::read(path).expect("Could not read file");

    let cr8 = exec::exec(instructions, CR8::new());

    cr8.debug()
}
