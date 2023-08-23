use std::{thread, time::Duration};

use cfg::{op::Operation, reg::Register};

use crate::cr8::CR8;

pub fn exec(instructions: Vec<u8>, mut cr8: CR8) -> CR8 {
    use Operation::*;

    for (i, inst) in instructions.into_iter().enumerate() {
        cr8.mem[i] = inst;
    }

    loop {
        let (pcl, pch) = cr8.pc();
        let pc = ((pch as u16) << 8) | pcl as u16;

        let inst = cr8.mem[pc as usize];

        if inst == 255 {
            // Halt simulator
            break cr8;
        }

        let op = inst >> 4;
        let op = Operation::from(op);

        let is_imm = (inst >> 3) & 0b1;
        let is_imm = is_imm == 1;

        let reg_id = inst & 0b111;
        let reg = Register::from(reg_id as usize);

        let mut fetch_imm8_0: u8 = 0;
        let mut fetch_imm8_1: u8 = 0;
        let mut fetch_reg: Register = Register::A;

        if is_imm {
            if op == Operation::LW || op == Operation::SW {
                fetch_imm8_0 = cr8.mem[(pc + 1) as usize];
                fetch_imm8_1 = cr8.mem[(pc + 2) as usize];
                cr8.tick_pc();
                cr8.tick_pc();
            } else if op != Operation::POP {
                fetch_imm8_0 = cr8.mem[(pc + 1) as usize];
                cr8.tick_pc();
            }
        } else {
            if op == Operation::MOV || op as u8 >= 6 {
                fetch_reg = Register::from(cr8.mem[(pc + 1) as usize]);
                cr8.tick_pc();
            }
        }

        match (op, is_imm) {
            (LW, true) => cr8.lw_imm16(reg, (fetch_imm8_0, fetch_imm8_1)),
            (LW, false) => cr8.lw_hl(reg),
            (SW, true) => cr8.sw_imm16(reg, (fetch_imm8_0, fetch_imm8_1)),
            (SW, false) => cr8.sw_hl(reg),
            (MOV, true) => cr8.mov_imm8(reg, fetch_imm8_0),
            (MOV, false) => cr8.mov_reg(reg, fetch_reg),
            (PUSH, true) => cr8.push_imm8(fetch_imm8_0),
            (PUSH, false) => cr8.push_reg(fetch_reg),
            (POP, _) => cr8.pop(reg),
            (JNZ, true) => cr8.jnz_imm8(fetch_imm8_0),
            (JNZ, false) => cr8.jnz_reg(reg),
            (IN, true) => cr8.in_imm8(reg, fetch_imm8_0),
            (IN, false) => cr8.in_reg(reg, fetch_reg),
            (OUT, true) => cr8.out_imm8(reg, fetch_imm8_0),
            (OUT, false) => cr8.out_reg(reg, fetch_reg),
            (CMP, true) => cr8.cmp_imm8(reg, fetch_imm8_0),
            (CMP, false) => cr8.cmp_reg(reg, fetch_reg),
            (ADC, true) => cr8.adc_imm8(reg, fetch_imm8_0),
            (ADC, false) => cr8.adc_reg(reg, fetch_reg),
            (SBB, true) => cr8.sbb_imm8(reg, fetch_imm8_0),
            (SBB, false) => cr8.sbb_reg(reg, fetch_reg),
            (OR, true) => cr8.or_imm8(reg, fetch_imm8_0),
            (OR, false) => cr8.or_reg(reg, fetch_reg),
            (NOR, true) => cr8.nor_imm8(reg, fetch_imm8_0),
            (NOR, false) => cr8.nor_reg(reg, fetch_reg),
            (AND, true) => cr8.and_imm8(reg, fetch_imm8_0),
            (AND, false) => cr8.and_reg(reg, fetch_reg),
        };

        cr8.tick_pc();
        thread::sleep(Duration::from_millis(500));
    }
}
