use cfg::{op::Operation, reg::Register};

use crate::cr8::CR8;

pub fn exec(instructions: Vec<u8>, mut cr8: CR8) -> CR8 {
    use Operation::*;

    let mut cnt = 0;

    for (i, inst) in instructions.iter().enumerate() {
        if cnt > 0 {
            cnt -= 1;
            continue;
        }

        let op = inst >> 4;
        let op = Operation::from(op);

        let is_imm = (inst << 4) >> 7;
        let is_imm = is_imm == 1;

        let reg_id = (inst << 5) >> 5;
        let reg = Register::from(reg_id);

        let mut fetch_imm8: u8 = 0;
        let mut fetch_imm16: u16 = 0;
        let mut fetch_reg: Register = Register::A;

        if is_imm {
            if op == Operation::LW || op == Operation::SW {
                let l = instructions[i + 1] as u16;
                let h = instructions[i + 2] as u16;
                fetch_imm16 = (h << 8) | l;
                cnt += 2;
            } else if op != Operation::POP {
                fetch_imm8 = instructions[i + 1] as u8;
                cnt += 1;
            }
        } else {
            if op == Operation::MOV || op as u8 >= 6 {
                fetch_reg = Register::from(instructions[i + 1]);
                cnt += 1;
            }
        }

        match (op, is_imm) {
            (LW, true) => cr8.lw_imm16(reg, fetch_imm16),
            (LW, false) => cr8.lw_hl(reg),
            (SW, true) => cr8.sw_imm16(fetch_imm16, reg),
            (SW, false) => cr8.sw_hl(reg),
            (MOV, true) => cr8.mov_imm8(reg, fetch_imm8),
            (MOV, false) => cr8.mov_reg(reg, fetch_reg),
            (PUSH, true) => cr8.push_imm8(fetch_imm8),
            (PUSH, false) => cr8.push_reg(fetch_reg),
            (POP, _) => cr8.pop(reg),
            (JNZ, true) => cr8.jnz_imm8(fetch_imm8),
            (JNZ, false) => cr8.jnz_reg(reg),
            (INB, true) => cr8.inb_imm8(reg, fetch_imm8),
            (INB, false) => cr8.inb_reg(reg, fetch_reg),
            (OUTB, true) => cr8.outb_imm8(reg, fetch_imm8),
            (OUTB, false) => cr8.outb_reg(reg, fetch_reg),
            (CMP, true) => cr8.cmp_imm8(reg, fetch_imm8),
            (CMP, false) => cr8.cmp_reg(reg, fetch_reg),
            (ADC, true) => cr8.adc_imm8(reg, fetch_imm8),
            (ADC, false) => cr8.adc_reg(reg, fetch_reg),
            (SBB, true) => cr8.sbb_imm8(reg, fetch_imm8),
            (SBB, false) => cr8.sbb_reg(reg, fetch_reg),
            (OR, true) => cr8.or_imm8(reg, fetch_imm8),
            (OR, false) => cr8.or_reg(reg, fetch_reg),
            (NOR, true) => cr8.nor_imm8(reg, fetch_imm8),
            (NOR, false) => cr8.nor_reg(reg, fetch_reg),
            (AND, true) => cr8.and_imm8(reg, fetch_imm8),
            (AND, false) => cr8.and_reg(reg, fetch_reg),
        };
    }

    return cr8;
}
