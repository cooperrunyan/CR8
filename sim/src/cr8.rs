use std::num::Wrapping;

use cr8_cfg::{
    mem::{PROGRAM_COUNTER, STACK, STACK_END, STACK_POINTER},
    reg::Register,
};

use crate::device::Device;

pub struct CR8 {
    reg: [u8; 8],
    mem: [u8; 65536],
    dev: Vec<Device>,
}

impl CR8 {
    pub fn new() -> Self {
        let mut cr8 = Self {
            reg: [0; 8],
            mem: [0; 65536],
            dev: vec![],
        };

        // initialize stack pointer;
        cr8.set_sp(STACK);
        cr8
    }

    fn hl(&self) -> u16 {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        ((h as u16) << 8) | l as u16
    }

    fn sp(&self) -> u16 {
        let spl = self.mem[STACK_POINTER as usize];
        let sph = self.mem[(STACK_POINTER + 1) as usize];

        ((sph as u16) << 8) | spl as u16
    }

    fn set_sp(&mut self, hl: u16) {
        let sph = (hl >> 8) as u8;
        let spl = ((hl << 8) >> 8) as u8;

        self.mem[STACK_POINTER as usize] = spl;
        self.mem[(STACK_POINTER + 1) as usize] = sph;
    }

    pub fn lw_imm16(&mut self, to: Register, imm16: u16) {
        self.reg[to as usize] = self.mem[imm16 as usize];
    }

    pub fn lw_hl(&mut self, to: Register) {
        self.reg[to as usize] = self.mem[self.hl() as usize];
    }

    pub fn sw_hl(&mut self, from: Register) {
        self.mem[self.hl() as usize] = self.reg[from as usize];
    }

    pub fn sw_imm16(&mut self, imm16: u16, from: Register) {
        self.mem[imm16 as usize] = self.reg[from as usize];
    }

    pub fn mov_reg(&mut self, to: Register, from: Register) {
        self.reg[to as usize] = self.reg[from as usize];
    }

    pub fn mov_imm8(&mut self, to: Register, imm8: u8) {
        self.reg[to as usize] = imm8;
    }

    pub fn push_imm8(&mut self, imm8: u8) {
        let sptr = self.sp();

        if sptr >= STACK_END {
            panic!("Stack overflow");
        }

        self.set_sp(sptr + 1);

        self.mem[(sptr + 1) as usize] = imm8;
    }

    pub fn push_reg(&mut self, reg: Register) {
        self.push_imm8(self.reg[reg as usize]);
    }

    pub fn pop(&mut self, reg: Register) {
        let sptr = self.sp();

        if sptr <= STACK {
            panic!("Cannot pop empty stack");
        }

        self.set_sp(sptr - 1);

        self.reg[reg as usize] = self.mem[(sptr - 1) as usize].clone();
        self.mem[(sptr - 1) as usize] = 0;
    }

    pub fn jnz_imm8(&mut self, imm8: u8) {
        if imm8 == 0 {
            return;
        }

        self.mem[PROGRAM_COUNTER as usize] = self.reg[Register::L as usize];
        self.mem[(PROGRAM_COUNTER + 1) as usize] = self.reg[Register::H as usize];
    }

    pub fn jnz_reg(&mut self, reg: Register) {
        self.jnz_imm8(self.reg[reg as usize]);
    }

    pub fn inb_reg(&mut self, dev_id: u8, reg: Register) {
        let i = (|| {
            for (i, dev) in self.dev.iter().enumerate() {
                if dev.id == dev_id {
                    return i;
                }
            }
            panic!("Attempted to address unpresent device");
        })();

        self.dev[i].send.call((&self.dev[i], self));
    }

    pub fn outb_imm8(&mut self, dev_id: u8, imm8: u8) {
        let i = (|| {
            for (i, dev) in self.dev.iter().enumerate() {
                if dev.id == dev_id {
                    return i;
                }
            }
            panic!("Attempted to address unpresent device");
        })();

        self.dev[i].recieve.call((&self.dev[i], self, imm8));
    }

    pub fn outb_reg(&mut self, dev_id: u8, reg: Register) {
        self.outb_imm8(dev_id, self.reg[reg as usize]);
    }

    pub fn cmp_imm8(&mut self, lhs: Register, imm8: u8) {
        let diff = (self.reg[lhs as usize] as i16) - imm8 as i16;
        let mut f = 0;

        if diff == 0 {
            f = f | 0b0010;
        }

        if diff < 0 {
            f = f | 0b0001;
        }

        self.reg[Register::F as usize] = f;
    }

    pub fn cmp_reg(&mut self, lhs: Register, reg: Register) {
        self.cmp_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn adc_imm8(&mut self, lhs: Register, imm8: u8) {
        let f = self.reg[Register::F as usize];
        let cf = (f >> 2) & 1;

        let mut of = false;

        let res = Wrapping(self.reg[lhs as usize]) + Wrapping(imm8) + Wrapping(cf);
        let res = res.0;

        if res < self.reg[lhs as usize] || res < imm8 || res < cf {
            self.reg[Register::F as usize] = self.reg[Register::F as usize] | 0b0100;
        }

        self.reg[lhs as usize] = res;
    }

    pub fn adc_reg(&mut self, lhs: Register, reg: Register) {
        self.adc_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn sbb_imm8(&mut self, lhs: Register, imm8: u8) {
        self.adc_imm8(lhs, !imm8 + 1);
    }

    pub fn sbb_reg(&mut self, lhs: Register, reg: Register) {
        self.sbb_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn or_imm8(&mut self, lhs: Register, imm8: u8) {
        self.reg[lhs as usize] = self.reg[lhs as usize] | imm8;
    }

    pub fn or_reg(&mut self, lhs: Register, reg: Register) {
        self.or_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn nor_imm8(&mut self, lhs: Register, imm8: u8) {
        self.reg[lhs as usize] = !(self.reg[lhs as usize] | imm8);
    }

    pub fn nor_reg(&mut self, lhs: Register, reg: Register) {
        self.nor_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn and_imm8(&mut self, lhs: Register, imm8: u8) {
        self.reg[lhs as usize] = self.reg[lhs as usize] & imm8;
    }

    pub fn and_reg(&mut self, lhs: Register, reg: Register) {
        self.and_imm8(lhs, self.reg[reg as usize]);
    }

    pub fn dev_add(&mut self, dev: Device) {
        let mut dev = dev;
        dev.id = self.dev.len() as u8;
        self.dev.push(dev);
    }

    pub fn dev_rm(&mut self, id: u8) {
        let i = (|| {
            for (i, dev) in self.dev.iter().enumerate() {
                if dev.id == id {
                    return i;
                }
            }
            panic!("Attempted to remove unpresent device");
        })();

        let _ = self.dev[i].drop.call((&self.dev[i], self));
        self.dev.remove(i);
    }

    pub fn debug(&self) {
        println!("A: {}", self.reg[Register::A as usize]);
        println!("B: {}", self.reg[Register::B as usize]);
        println!("C: {}", self.reg[Register::C as usize]);
        println!("D: {}", self.reg[Register::D as usize]);
        println!("Z: {}", self.reg[Register::Z as usize]);
        println!("HL: {}", self.hl());
        println!("[HL]: {}", self.mem[self.hl() as usize]);
        println!("SP: {}", self.sp() - STACK);
        println!("[SP]: {}", self.mem[self.sp() as usize]);
        println!();
        println!("Devices:");

        for (i, dev) in self.dev.iter().enumerate() {
            println!("  {i}: {}", dev.name);
        }

        println!();
        let f = self.reg[Register::F as usize];
        let lf = f & 1;
        let ef = (f >> 1) & 1;
        let cf = (f >> 2) & 1;
        let zf = (f >> 3) & 1;

        println!();
        println!("LF: {}", lf == 1);
        println!("EF: {}", ef == 1);
        println!("CF: {}", cf == 1);
        println!("ZF: {}", zf == 1);
    }
}
