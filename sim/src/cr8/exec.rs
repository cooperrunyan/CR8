use std::{thread, time::Duration};

use asm::{mem::STACK, op::Operation, reg::Register};

use super::{join, CR8};

impl CR8 {
    pub fn run(&mut self) -> Result<(), String> {
        use Operation::*;

        loop {
            if let Some(dev) = self.dev.get(&0) {
                let status = dev.inspect();

                if status == 0x01 {
                    break;
                }
                if status >> 1 == 1 {
                    let ptr = STACK as usize;
                    let stack_frame = &self.mem[ptr..ptr + 10];
                    println!();
                    println!("SIGPEEKSTACK:");
                    dbg!(stack_frame);
                    println!();
                }
            }

            let inst = self.mem[join(self.pc()) as usize];

            if inst == 0xFF {
                // Halt simulator
                break;
            }

            let op = Operation::try_from(inst >> 4)?;
            let is_imm = (inst & 0b00001000) == 0b00001000;
            let reg_bits = inst & 0b00000111;

            let b0: u8 = *self.mem.get((join(self.pc()) + 1) as usize).unwrap_or(&0);
            let b1: u8 = *self.mem.get((join(self.pc()) + 2) as usize).unwrap_or(&0);

            let ticks = match (op, is_imm) {
                (LW, true) => self.lw_imm16(Register::try_from(reg_bits)?, (b0, b1)),
                (LW, false) => self.lw_hl(Register::try_from(reg_bits)?),
                (SW, true) => self.sw_imm16((b0, b1), Register::try_from(reg_bits)?),
                (SW, false) => self.sw_hl(Register::try_from(reg_bits)?),
                (MOV, true) => self.mov_imm8(Register::try_from(reg_bits)?, b0),
                (MOV, false) => {
                    self.mov_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (PUSH, true) => self.push_imm8(b0),
                (PUSH, false) => self.push_reg(Register::try_from(reg_bits)?),
                (POP, _) => self.pop(Register::try_from(reg_bits)?),
                (JNZ, true) => self.jnz_imm8(b0),
                (JNZ, false) => self.jnz_reg(Register::try_from(reg_bits)?),
                (IN, true) => self.in_imm8(Register::try_from(reg_bits)?, b0),
                (IN, false) => self.in_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?),
                (OUT, true) => self.out_imm8(b0, Register::try_from(reg_bits)?),
                (OUT, false) => {
                    self.out_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (CMP, true) => self.cmp_imm8(Register::try_from(reg_bits)?, b0),
                (CMP, false) => {
                    self.cmp_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (ADC, true) => self.adc_imm8(Register::try_from(reg_bits)?, b0),
                (ADC, false) => {
                    self.adc_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (SBB, true) => self.sbb_imm8(Register::try_from(reg_bits)?, b0),
                (SBB, false) => {
                    self.sbb_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (OR, true) => self.or_imm8(Register::try_from(reg_bits)?, b0),
                (OR, false) => self.or_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?),
                (NOR, true) => self.nor_imm8(Register::try_from(reg_bits)?, b0),
                (NOR, false) => {
                    self.nor_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
                (AND, true) => self.and_imm8(Register::try_from(reg_bits)?, b0),
                (AND, false) => {
                    self.and_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
                }
            };

            for _ in 0..ticks {
                self.inc_pc()
            }

            thread::sleep(Duration::from_millis(self.tick_rate));
        }

        Ok(())
    }
}
