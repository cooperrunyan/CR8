use asm::op::Operation;
use log::trace;
use std::sync::RwLock;

use anyhow::{anyhow, bail, Result};
use asm::reg::Register;

use crate::cr8::Joinable;
use crate::devices::Devices;

use super::mem::Mem;
use super::{CR8, STACK, STACK_END};
use Operation as O;

impl CR8 {
    /// Decide which [Operation] to run
    pub fn delegate(
        &mut self,
        mem: &RwLock<Mem>,
        dev: &RwLock<Devices>,
        bytes: [u8; 4],
    ) -> Result<u8> {
        let instruction =
            Operation::try_from(bytes[0] >> 4).map_err(|_| anyhow!("Invalid operation"))?;
        let is_imm = (bytes[0] >> 3) & 1 == 1;

        match instruction {
            O::MOV => self.mov(is_imm, bytes),
            O::JNZ => self.jnz(is_imm, bytes),
            O::JMP => self.jmp(is_imm, bytes),
            O::LW => self.lw(mem, is_imm, bytes),
            O::SW => self.sw(mem, is_imm, bytes),
            O::PUSH => self.push(mem, is_imm, bytes),
            O::POP => self.pop(mem, bytes),
            O::IN => self.r#in(dev, is_imm, bytes),
            O::OUT => self.out(dev, is_imm, bytes),
            O::ADC => self.add(is_imm, bytes),
            O::SBB => self.sub(is_imm, bytes),
            O::CMP => self.cmp(is_imm, bytes),
            O::AND => self.and(is_imm, bytes),
            O::OR => self.or(is_imm, bytes, false),
            O::NOR => self.or(is_imm, bytes, true),
            O::BANK => self.bank(mem, is_imm, bytes),
        }
    }

    /// LW: (see README.md)
    fn lw(&mut self, mem: &RwLock<Mem>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let (to, addr, sz) = match is_imm {
            false => (bytes[0] & 0b111, self.xy(), 1),
            true => (bytes[0] & 0b111, (bytes[1], bytes[2]).join(), 3),
        };
        trace!("{:04x}: LW {to:#?} {addr:04x}", self.pc);
        self.reg[to as usize] = {
            let mem = mem.read().unwrap();
            mem.get(addr)?
        };
        Ok(sz)
    }

    /// SW: (see README.md)
    fn sw(&mut self, mem: &RwLock<Mem>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let (val, addr, sz) = match is_imm {
            false => (self.reg[(bytes[0] & 0b111) as usize], self.xy(), 1),
            true => (
                self.reg[(bytes[0] & 0b111) as usize],
                (bytes[1], bytes[2]).join(),
                3,
            ),
        };
        trace!("{:04x}: SW {val:#?} {addr:04x}", self.pc);
        let mut mem = mem.write().unwrap();
        mem.set(addr, val)?;
        Ok(sz)
    }

    /// MOV: (see README.md)
    fn mov(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let (into, val, sz) = match is_imm {
            true => (bytes[0] & 0b111, bytes[1], 2),
            false => (bytes[0] & 0b111, self.reg[(bytes[1] & 0b111) as usize], 2),
        };
        trace!("{:04x}: MOV {into:#?}, {val:02x} | {val:?}", self.pc);
        self.reg[into as usize] = val;
        Ok(sz)
    }

    /// PUSH: (see README.md)
    fn push(&mut self, mem: &RwLock<Mem>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        if self.sp >= STACK_END {
            bail!("Stack overflow");
        }

        self.sp += 1;

        let (val, sz) = match is_imm {
            false => (self.reg[(bytes[0] & 0b111) as usize], 1),
            true => (bytes[1], 2),
        };
        {
            let mut mem = mem.write().unwrap();
            mem.set(self.sp, val)?;
        };

        trace!(
            "{:04x}: PUSHED: [{:04x}] {:02x}",
            self.pc,
            self.sp as i128 - STACK as i128,
            val,
        );
        Ok(sz)
    }

    /// POP: (see README.md)
    fn pop(&mut self, mem: &RwLock<Mem>, bytes: [u8; 4]) -> Result<u8> {
        if self.sp < STACK {
            bail!("Cannot pop empty stack");
        }

        let reg = bytes[0] & 0b111;

        {
            let mut mem = mem.write().unwrap();
            self.reg[reg as usize] = mem.get(self.sp)?;
            mem.set(self.sp, 0)?;
        };

        trace!(
            "{:04x}: POPPED: [{:04x}] {:?}",
            self.pc,
            self.sp - STACK,
            reg,
        );

        self.sp -= 1;
        Ok(1)
    }

    /// JNZ: (see README.md)
    fn jnz(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let condition = self.reg[(bytes[0] & 0b111) as usize];
        let (addr, sz) = match is_imm {
            false => (self.xy(), 1),
            true => ((bytes[1], bytes[2]).join(), 3),
        };
        if condition == 0 {
            trace!("{:04x}: No JNZ", self.pc);
            return Ok(sz);
        }

        let old = self.pc;

        self.pc = addr;

        trace!("{:04x}: JNZ to {:04x}", old, self.pc);
        Ok(0)
    }

    /// JMP: (see README.md)
    fn jmp(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let addr = match is_imm {
            false => self.xy(),
            true => (bytes[1], bytes[2]).join(),
        };

        let old = self.pc;

        self.pc = addr;

        trace!("{:04x}: JNZ to {:04x}", old, self.pc);
        Ok(0)
    }

    /// IN: (see README.md)
    fn r#in(&mut self, dev: &RwLock<Devices>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let into = bytes[0] & 0b111;
        let port = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: IN {into:#?}, {port:02x}", self.pc);
        let mut devices = dev.write().unwrap();
        self.reg[into as usize] = devices.receive(port)?;
        Ok(2)
    }

    /// OUT: (see README.md)
    fn out(&mut self, dev: &RwLock<Devices>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let send = bytes[0] & 0b111;
        let port = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: OUT {send:#?}, {port:02x}", self.pc);
        let mut devices = dev.write().unwrap();
        devices.send(port, self.reg[send as usize])?;
        Ok(2)
    }

    /// CMP: (see README.md)
    fn cmp(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let lhs = bytes[0] & 0b111;
        let rhs = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: CMP {lhs:#?}, {rhs:02x}", self.pc);

        let diff = (self.reg[lhs as usize] as i16) - (rhs as i16);
        let mut f = 0;

        if diff == 0 {
            f |= 0b0010;
        }

        if diff < 0 {
            f |= 0b0001;
        }

        self.reg[Register::F as usize] = f;
        Ok(2)
    }

    /// ADD: (see README.md)
    fn add(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let lhs = bytes[0] & 0b111;
        let rhs = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: ADD {lhs:#?}, {rhs:02x}", self.pc);

        let f = self.reg[Register::F as usize];
        let cf = (f >> 2) & 1;

        let (res, carry) = self.reg[lhs as usize].carrying_add(rhs, cf == 1);

        if carry {
            self.reg[Register::F as usize] |= 0b0100;
        } else {
            self.reg[Register::F as usize] &= 0b1011;
        }

        self.reg[lhs as usize] = res;
        Ok(2)
    }

    /// SUB: (see README.md)
    fn sub(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let lhs = bytes[0] & 0b111;
        let rhs = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: SUB {lhs:#?}, {rhs:02x}", self.pc);

        let f = self.reg[Register::F as usize];
        let bf = (f >> 3) & 1;

        let (res, borrow) = self.reg[lhs as usize].borrowing_sub(rhs, bf == 1);

        if borrow {
            self.reg[Register::F as usize] |= 0b1000;
        } else {
            self.reg[Register::F as usize] &= 0b0111;
        }

        self.reg[lhs as usize] = res;
        Ok(2)
    }

    /// OR: (see README.md)
    fn or(&mut self, is_imm: bool, bytes: [u8; 4], not: bool) -> Result<u8> {
        let lhs = bytes[0] & 0b111;
        let rhs = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: OR {lhs:#?}, {rhs:02x}", self.pc);
        self.reg[lhs as usize] |= rhs;
        if not {
            self.reg[lhs as usize] = !self.reg[lhs as usize];
        }
        Ok(2)
    }

    /// AND: (see README.md)
    fn and(&mut self, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let lhs = bytes[0] & 0b111;
        let rhs = match is_imm {
            true => bytes[1],
            false => self.reg[(bytes[1] & 0b111) as usize],
        };
        trace!("{:04x}: AND {lhs:#?}, {rhs:02x}", self.pc);
        self.reg[lhs as usize] &= rhs;
        Ok(2)
    }

    /// MOV: (see README.md)
    fn bank(&mut self, mem: &RwLock<Mem>, is_imm: bool, bytes: [u8; 4]) -> Result<u8> {
        let (val, sz) = match is_imm {
            false => (bytes[0] & 0b111, 1),
            true => (bytes[1], 2),
        };
        trace!("{:04x}: BANK {val:02x} | {val:?}", self.pc);
        mem.write().unwrap().select(val)?;
        Ok(sz)
    }
}
