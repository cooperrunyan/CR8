use std::num::Wrapping;

use cfg::{
    mem::{PROGRAM_COUNTER, STACK, STACK_END},
    reg::Register,
};

use super::{join, split, CR8};

impl CR8 {
    pub(super) fn lw_imm16(&mut self, to: Register, i: (u8, u8)) {
        let addr = join(i);
        // println!("LW {to:#?}, {addr:#?}");
        self.reg[to as usize] = self.mem[addr as usize];
    }

    pub(super) fn lw_hl(&mut self, to: Register) {
        let addr = join(self.hl());
        // println!("LW {to:#?}, {}", addr);
        self.reg[to as usize] = self.mem[addr as usize];
    }

    pub(super) fn sw_hl(&mut self, from: Register) {
        // println!("SW {from:#?}, {}", join(self.hl()));
        self.mem[join(self.hl()) as usize] = self.reg[from as usize];
    }

    pub(super) fn sw_imm16(&mut self, i: (u8, u8), from: Register) {
        // println!("SW {from:#?}, {}", join(i));
        self.mem[join(i) as usize] = self.reg[from as usize];
    }

    pub(super) fn mov_reg(&mut self, to: Register, from: Register) {
        // println!("MOV {to:#?}, {from:#?}");

        self.reg[to as usize] = self.reg[from as usize];
    }

    pub(super) fn mov_imm8(&mut self, to: Register, imm8: u8) {
        // println!("MOV {to:#?}, {imm8:#?}");
        self.reg[to as usize] = imm8;
    }

    pub(super) fn push_imm8(&mut self, imm8: u8) {
        let sptr = join(self.sp());

        if sptr >= STACK_END {
            panic!("Stack overflow");
        }

        self.set_sp(split(sptr + 1));

        self.mem[join(self.sp()) as usize] = imm8;

        // println!("PUSHED: [{}] {}", join(self.sp()) - STACK, imm8);
    }

    pub(super) fn push_reg(&mut self, reg: Register) {
        self.push_imm8(self.reg[reg as usize]);
    }

    pub(super) fn pop(&mut self, reg: Register) {
        let sptr = join(self.sp());

        if sptr < STACK {
            panic!("Cannot pop empty stack");
        }

        self.reg[reg as usize] = self.mem[sptr as usize];
        self.mem[sptr as usize] = 0;

        // println!("POPPED: [{}] {}", sptr - STACK, self.reg[reg as usize]);

        self.set_sp(split(sptr - 1));
    }

    pub(super) fn jnz_imm8(&mut self, imm8: u8) {
        if imm8 == 0 {
            return;
        }

        self.mem[PROGRAM_COUNTER as usize] = self.reg[Register::L as usize];
        self.mem[(PROGRAM_COUNTER + 1) as usize] = self.reg[Register::H as usize];

        // println!("JNZ {}, {imm8:#?}", join(self.pc()));
    }

    pub(super) fn jnz_reg(&mut self, reg: Register) {
        self.jnz_imm8(self.reg[reg as usize]);
    }

    pub(super) fn in_imm8(&mut self, into: Register, port: u8) {
        // println!("IN {into:#?}, {port:#?}");

        if let Some(dev) = self.dev.get_mut(&port) {
            self.reg[into as usize] = dev.send(&self.reg, &self.mem);
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
    }

    pub(super) fn in_reg(&mut self, into: Register, port: Register) {
        self.in_imm8(into, self.reg[port as usize]);
    }

    pub(super) fn out_imm8(&mut self, port: u8, send: Register) {
        // println!("OUT {send:#?}, {port:#?}");
        if let Some(dev) = self.dev.get_mut(&port) {
            dev.receive(&self.reg, &self.mem, self.reg[send as usize]);
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
    }

    pub(super) fn out_reg(&mut self, port: Register, send: Register) {
        self.out_imm8(self.reg[port as usize], send);
    }

    pub(super) fn cmp_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("CMP {lhs:#?}, {imm8:#?}");

        let diff = (self.reg[lhs as usize] as i16) - (imm8 as i16);
        let mut f = 0;

        if diff == 0 {
            f |= 0b0010;
        }

        if diff < 0 {
            f |= 0b0001;
        }

        self.reg[Register::F as usize] = f;
    }

    pub(super) fn cmp_reg(&mut self, lhs: Register, reg: Register) {
        self.cmp_imm8(lhs, self.reg[reg as usize]);
    }

    pub(super) fn adc_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("ADC {lhs:#?}, {imm8:#?}");

        let f = self.reg[Register::F as usize];
        let cf = (f >> 2) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + Wrapping(imm8) + Wrapping(cf);
        let res = res.0;

        if res < self.reg[lhs as usize] || res < imm8 || res < cf {
            self.reg[Register::F as usize] |= 0b0100;
        }

        self.reg[lhs as usize] = res;
    }

    pub(super) fn adc_reg(&mut self, lhs: Register, reg: Register) {
        self.adc_imm8(lhs, self.reg[reg as usize]);
    }

    pub(super) fn sbb_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("SBB {lhs:#?}, {imm8:#?}");

        let f = self.reg[Register::F as usize];
        let bf = (f >> 3) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + (Wrapping(!imm8 + 1) - Wrapping(bf));
        let res = res.0;

        if res > self.reg[lhs as usize] {
            self.reg[Register::F as usize] = 0b1000;
        }

        self.reg[lhs as usize] = res;
    }

    pub(super) fn sbb_reg(&mut self, lhs: Register, reg: Register) {
        self.sbb_imm8(lhs, self.reg[reg as usize]);
    }

    pub(super) fn or_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("OR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] |= imm8;
    }

    pub(super) fn or_reg(&mut self, lhs: Register, reg: Register) {
        self.or_imm8(lhs, self.reg[reg as usize]);
    }

    pub(super) fn nor_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("NOR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] = !(self.reg[lhs as usize] | imm8);
    }

    pub(super) fn nor_reg(&mut self, lhs: Register, reg: Register) {
        self.nor_imm8(lhs, self.reg[reg as usize]);
    }

    pub(super) fn and_imm8(&mut self, lhs: Register, imm8: u8) {
        // println!("AND {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] &= imm8;
    }

    pub(super) fn and_reg(&mut self, lhs: Register, reg: Register) {
        self.and_imm8(lhs, self.reg[reg as usize]);
    }
}
