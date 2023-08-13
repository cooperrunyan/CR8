#![feature(fn_traits)]

mod cr8;
mod device;
mod exec;

use cr8::CR8;

fn main() {
    let instructions = vec![
        0b00101000, 0b00000100, 0b00101001, 0b00000010, 0b10010000, 0b00000001,
    ];

    let cr8 = exec::exec(instructions, CR8::new());
    cr8.debug()
}
