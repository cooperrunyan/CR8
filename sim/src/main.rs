#![feature(fn_traits)]

mod args;
mod cr8;
mod device;

use cr8::CR8;

use crate::cr8::CR8Config;

#[cfg(test)]
mod test;

fn main() {
    let bin = args::parse().unwrap();

    let mut cr8 = CR8::new(CR8Config::builder().tick_rate(0).mem(bin).build());

    cr8.run();
    cr8.debug()
}
