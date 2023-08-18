#![feature(fn_traits)]

mod args;
mod cr8;
mod device;
mod exec;

use cr8::CR8;

fn main() {
    let bin = args::parse().unwrap();

    let cr8 = exec::exec(bin, CR8::new());

    cr8.debug()
}
