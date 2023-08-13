#![feature(fn_traits)]

mod cr8;
mod device;

use cr8::CR8;
use cr8_cfg::reg::Register;

fn main() {
    let mut cr8 = CR8::new();

    cr8.mov_imm8(Register::A, 255);
    cr8.mov_imm8(Register::B, 5);
    cr8.push_reg(Register::A);
    cr8.adc_reg(Register::A, Register::B);
    cr8.debug();
}
