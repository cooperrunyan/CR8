#![feature(fn_traits)]

mod config;
mod cr8;
mod device;

use config::Config;
use cr8::CR8;

use crate::cr8::CR8Config;

fn main() {
    let config = Config::from_argv();

    let mut cr8 = CR8::new(
        CR8Config::builder()
            .tickrate(config.tickrate)
            .mem(config.bin)
            .debug(config.debug)
            .step(config.step)
            .build(),
    );

    match cr8.run() {
        Err(msg) => panic!("Error: {msg}"),
        _ => {}
    };
    cr8.debug()
}
