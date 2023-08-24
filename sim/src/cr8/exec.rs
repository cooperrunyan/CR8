use std::{thread, time::Duration};

use cfg::{op::Operation, reg::Register};

use super::{join, CR8};

impl CR8 {
    pub fn run(&mut self) {
        use Operation::*;

        loop {
            if let Some(dev) = self.dev.get(&0) {
                if dev.inspect() == 0x01 {
                    break;
                }
            }

            let inst = self.mem[join(self.pc()) as usize];

            if inst == 0xFF {
                // Halt simulator
                break;
            }

            let op = Operation::from(inst >> 1);
            let is_imm = (inst & 0b01) == 1;

            let b0: u8 = *self.mem.get((join(self.pc()) + 1) as usize).unwrap_or(&0);
            let b1: u8 = *self.mem.get((join(self.pc()) + 2) as usize).unwrap_or(&0);
            let b2: u8 = *self.mem.get((join(self.pc()) + 3) as usize).unwrap_or(&0);

            match (op, is_imm) {
                (LW | SW | POP, false) => {
                    self.inc_pc();
                    self.inc_pc();
                }
                (LW | SW, true) => {
                    self.inc_pc();
                    self.inc_pc();
                    self.inc_pc();
                    self.inc_pc();
                }
                (PUSH | JNZ, _) => {
                    self.inc_pc();
                    self.inc_pc();
                }
                (MOV | IN | OUT | CMP | ADC | SBB | OR | NOR | AND, false) => {
                    self.inc_pc();
                    self.inc_pc();
                    self.inc_pc();
                }
                (MOV | IN | OUT | CMP | ADC | SBB | OR | NOR | AND, true) => {
                    self.inc_pc();
                    self.inc_pc();
                    self.inc_pc();
                }
                _ => {}
            };

            match (op, is_imm) {
                (LW, true) => self.lw_imm16(Register::from(b0), (b1, b2)),
                (LW, false) => self.lw_hl(Register::from(b0)),
                (SW, true) => self.sw_imm16((b1, b2), Register::from(b0)),
                (SW, false) => self.sw_hl(Register::from(b0)),
                (MOV, true) => self.mov_imm8(Register::from(b0), b1),
                (MOV, false) => self.mov_reg(Register::from(b0), Register::from(b1)),
                (PUSH, true) => self.push_imm8(b0),
                (PUSH, false) => self.push_reg(Register::from(b0)),
                (POP, _) => self.pop(Register::from(b0)),
                (JNZ, true) => self.jnz_imm8(b0),
                (JNZ, false) => self.jnz_reg(Register::from(b0)),
                (IN, true) => self.in_imm8(Register::from(b0), b1),
                (IN, false) => self.in_reg(Register::from(b0), Register::from(b1)),
                (OUT, true) => self.out_imm8(b0, Register::from(b1)),
                (OUT, false) => self.out_reg(Register::from(b0), Register::from(b1)),
                (CMP, true) => self.cmp_imm8(Register::from(b0), b1),
                (CMP, false) => self.cmp_reg(Register::from(b0), Register::from(b1)),
                (ADC, true) => self.adc_imm8(Register::from(b0), b1),
                (ADC, false) => self.adc_reg(Register::from(b0), Register::from(b1)),
                (SBB, true) => self.sbb_imm8(Register::from(b0), b1),
                (SBB, false) => self.sbb_reg(Register::from(b0), Register::from(b1)),
                (OR, true) => self.or_imm8(Register::from(b0), b1),
                (OR, false) => self.or_reg(Register::from(b0), Register::from(b1)),
                (NOR, true) => self.nor_imm8(Register::from(b0), b1),
                (NOR, false) => self.nor_reg(Register::from(b0), Register::from(b1)),
                (AND, true) => self.and_imm8(Register::from(b0), b1),
                (AND, false) => self.and_reg(Register::from(b0), Register::from(b1)),
            };

            thread::sleep(Duration::from_millis(self.tick_rate));
        }
    }
}
