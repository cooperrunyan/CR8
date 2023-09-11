use std::num::Wrapping;

use asm::reg::Register;

use super::{join, split, CR8, PROGRAM_COUNTER, STACK, STACK_END};
macro_rules! cr8debug {
    ($self:ident, $msg:expr $(,$args:expr)*) => {
        if $self.debug {
            println!($msg $(,$args)*);
        }
    }
}

impl CR8 {
    pub(super) fn lw_imm16(&mut self, to: Register, i: (u8, u8)) -> u8 {
        let addr = join(i);
        cr8debug!(self, "LW {to:#?}, {addr:#?}");
        self.reg[to as usize] = self.mem[addr as usize];
        3
    }

    pub(super) fn lw_hl(&mut self, to: Register) -> u8 {
        let addr = join(self.hl());
        cr8debug!(self, "LW {to:#?}, {}", addr);
        self.reg[to as usize] = self.mem[addr as usize];
        1
    }

    pub(super) fn sw_hl(&mut self, from: Register) -> u8 {
        cr8debug!(self, "SW {from:#?}, {}", join(self.hl()));
        self.mem[join(self.hl()) as usize] = self.reg[from as usize];
        1
    }

    pub(super) fn sw_imm16(&mut self, i: (u8, u8), from: Register) -> u8 {
        cr8debug!(self, "SW {from:#?}, {}", join(i));
        self.mem[join(i) as usize] = self.reg[from as usize];
        3
    }

    pub(super) fn mov_reg(&mut self, to: Register, from: Register) -> u8 {
        cr8debug!(self, "MOV {to:#?}, {from:#?}");

        self.reg[to as usize] = self.reg[from as usize];
        2
    }

    pub(super) fn mov_imm8(&mut self, to: Register, imm8: u8) -> u8 {
        cr8debug!(self, "MOV {to:#?}, {imm8:#?}");
        self.reg[to as usize] = imm8;
        2
    }

    pub(super) fn push_imm8(&mut self, imm8: u8) -> u8 {
        let sptr = join(self.sp());

        if sptr >= STACK_END {
            panic!("Stack overflow");
        }

        self.set_sp(split(sptr + 1));

        self.mem[join(self.sp()) as usize] = imm8;

        cr8debug!(self, "PUSHED: [{}] {}", join(self.sp()) - STACK, imm8);
        2
    }

    pub(super) fn push_reg(&mut self, reg: Register) -> u8 {
        self.push_imm8(self.reg[reg as usize]);
        1
    }

    pub(super) fn pop(&mut self, reg: Register) -> u8 {
        let sptr = join(self.sp());

        if sptr < STACK {
            panic!("Cannot pop empty stack");
        }

        self.reg[reg as usize] = self.mem[sptr as usize];
        self.mem[sptr as usize] = 0;

        cr8debug!(
            self,
            "POPPED: [{}] {}",
            sptr - STACK,
            self.reg[reg as usize]
        );

        self.set_sp(split(sptr - 1));
        1
    }

    pub(super) fn jnz_imm8(&mut self, imm8: u8) -> u8 {
        if imm8 == 0 {
            cr8debug!(self, "No JNZ");
            return 2;
        }

        self.mem[PROGRAM_COUNTER as usize] = self.reg[Register::L as usize];
        self.mem[(PROGRAM_COUNTER + 1) as usize] = self.reg[Register::H as usize];

        cr8debug!(self, "JNZ to {}", join(self.pc()));
        0
    }

    pub(super) fn jnz_reg(&mut self, reg: Register) -> u8 {
        let v = self.reg[reg as usize];
        self.jnz_imm8(self.reg[reg as usize]);
        if v == 0 {
            return 1;
        }
        return 0;
    }

    pub(super) fn in_imm8(&mut self, into: Register, port: u8) -> u8 {
        cr8debug!(self, "IN {into:#?}, {port:#?}");

        if let Some(dev) = self.dev.get_mut(&port) {
            self.reg[into as usize] = dev.send(&self.reg, &self.mem);
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
        2
    }

    pub(super) fn in_reg(&mut self, into: Register, port: Register) -> u8 {
        self.in_imm8(into, self.reg[port as usize]);
        2
    }

    pub(super) fn out_imm8(&mut self, port: u8, send: Register) -> u8 {
        cr8debug!(self, "OUT {send:#?}, {port:#?}");
        if let Some(dev) = self.dev.get_mut(&port) {
            dev.receive(&self.reg, &self.mem, self.reg[send as usize]);
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
        2
    }

    pub(super) fn out_reg(&mut self, port: Register, send: Register) -> u8 {
        self.out_imm8(self.reg[port as usize], send);
        2
    }

    pub(super) fn cmp_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "CMP {lhs:#?}, {imm8:#?}");

        let diff = (self.reg[lhs as usize] as i16) - (imm8 as i16);
        let mut f = 0;

        if diff == 0 {
            f |= 0b0010;
        }

        if diff < 0 {
            f |= 0b0001;
        }

        self.reg[Register::F as usize] = f;
        2
    }

    pub(super) fn cmp_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.cmp_imm8(lhs, self.reg[reg as usize]);
        2
    }

    pub(super) fn adc_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "ADC {lhs:#?}, {imm8:#?}");

        let f = self.reg[Register::F as usize];
        let cf = (f >> 2) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + Wrapping(imm8) + Wrapping(cf);
        let res = res.0;

        if res < self.reg[lhs as usize] || res < imm8 || res < cf {
            self.reg[Register::F as usize] |= 0b0100;
        }

        self.reg[lhs as usize] = res;
        2
    }

    pub(super) fn adc_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.adc_imm8(lhs, self.reg[reg as usize]);
        2
    }

    pub(super) fn sbb_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "SBB {lhs:#?}, {imm8:#?}");

        let f = self.reg[Register::F as usize];
        let bf = (f >> 3) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + (Wrapping(!imm8) + Wrapping(1) - Wrapping(bf));
        let res = res.0;

        if res > self.reg[lhs as usize] {
            self.reg[Register::F as usize] = 0b1000;
        }

        self.reg[lhs as usize] = res;
        2
    }

    pub(super) fn sbb_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.sbb_imm8(lhs, self.reg[reg as usize]);
        2
    }

    pub(super) fn or_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "OR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] |= imm8;
        2
    }

    pub(super) fn or_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.or_imm8(lhs, self.reg[reg as usize]);
        2
    }

    pub(super) fn nor_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "NOR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] = !(self.reg[lhs as usize] | imm8);
        2
    }

    pub(super) fn nor_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.nor_imm8(lhs, self.reg[reg as usize]);
        2
    }

    pub(super) fn and_imm8(&mut self, lhs: Register, imm8: u8) -> u8 {
        cr8debug!(self, "AND {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] &= imm8;
        2
    }

    pub(super) fn and_reg(&mut self, lhs: Register, reg: Register) -> u8 {
        self.and_imm8(lhs, self.reg[reg as usize]);
        2
    }
}
