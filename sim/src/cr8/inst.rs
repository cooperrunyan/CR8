use log::trace;
use std::num::Wrapping;

use anyhow::{anyhow, Result};
use asm::reg::Register;

use crate::devices::{DeviceID, Devices};

use super::{CR8, STACK, STACK_END};

impl CR8 {
    pub fn lw_imm16(&mut self, to: Register, i: u16) -> Result<u8> {
        trace!("{:04x}: LW {to:#?} {i:04x}", self.pc);
        self.reg[to as usize] = self.mem.get(i)?;
        Ok(3)
    }

    pub fn lw_hl(&mut self, to: Register) -> Result<u8> {
        let addr = self.hl();
        trace!("{:04x}: LW {to:#?}, {:04x}", self.pc, addr,);
        self.reg[to as usize] = self.mem.get(addr)?;
        Ok(1)
    }

    pub fn sw_hl(&mut self, from: Register) -> Result<u8> {
        trace!("{:04x}: SW {:04x}, {from:#?}", self.pc, self.hl(),);
        self.mem.set(self.hl(), self.reg[from as usize])?;
        Ok(1)
    }

    pub fn sw_imm16(&mut self, i: u16, from: Register) -> Result<u8> {
        trace!("{:04x}: SW {:04x}, {from:#?}", self.pc, i,);
        self.mem.set(i, self.reg[from as usize].clone())?;
        Ok(3)
    }

    pub fn mov_reg(&mut self, to: Register, from: Register) -> Result<u8> {
        trace!("{:04x}: MOV {to:#?}, {from:#?}", self.pc);

        self.reg[to as usize] = self.reg[from as usize];
        Ok(2)
    }

    pub fn mov_imm8(&mut self, to: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: MOV {to:#?}, {imm8:02x} | {imm8:?}", self.pc);
        self.reg[to as usize] = imm8;
        Ok(2)
    }

    pub fn push_imm8(&mut self, imm8: u8) -> Result<u8> {
        if self.sp >= STACK_END {
            err!("Stack overflow");
        }

        self.sp += 1;
        self.mem.set(self.sp, imm8)?;

        trace!(
            "{:04x}: PUSHED: [{:04x}] {:02x}",
            self.pc,
            self.sp as i128 - STACK as i128,
            imm8,
        );
        Ok(2)
    }

    pub fn push_reg(&mut self, reg: Register) -> Result<u8> {
        self.push_imm8(self.reg[reg as usize])?;
        Ok(1)
    }

    pub fn pop(&mut self, reg: Register) -> Result<u8> {
        if self.sp < STACK {
            err!("Cannot pop empty stack");
        }

        self.reg[reg as usize] = self.mem.get(self.sp)?;
        self.mem.set(self.sp, 0)?;

        trace!(
            "{:04x}: POPPED: [{:04x}] {:?}",
            self.pc,
            self.sp - STACK,
            self.reg[reg as usize],
        );

        self.sp -= 1;
        Ok(1)
    }

    pub fn jnz_imm8(&mut self, imm8: u8) -> Result<u8> {
        if imm8 == 0 {
            trace!("{:04x}: No JNZ", self.pc);
            return Ok(2);
        }

        let old = self.pc;

        self.pc = self.hl();

        trace!("{:04x}: JNZ to {:04x}", old, self.pc);
        Ok(0)
    }

    pub fn jnz_reg(&mut self, reg: Register) -> Result<u8> {
        let v = self.reg[reg as usize];
        if v == 0 {
            return Ok(1);
        }
        self.jnz_imm8(self.reg[reg as usize])?;
        return Ok(0);
    }

    pub fn in_imm8(&mut self, devices: &Devices, into: Register, port: u8) -> Result<u8> {
        trace!("{:04x}: IN {into:#?}, {port:02x}", self.pc);

        if let Some(dev) = devices.get(DeviceID::from(port)) {
            self.reg[into as usize] = dev
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?
                .send()?;
        } else {
            self.debug();
            err!("No device connected to port: {port}");
        }
        Ok(2)
    }

    pub fn in_reg(&mut self, devices: &Devices, into: Register, port: Register) -> Result<u8> {
        self.in_imm8(devices, into, self.reg[port as usize])?;
        Ok(2)
    }

    pub fn out_imm8(&mut self, devices: &Devices, port: u8, send: Register) -> Result<u8> {
        trace!("{:04x}: OUT {send:#?}, {port:02x}", self.pc);
        if let Some(dev) = devices.get(DeviceID::from(port)) {
            dev.lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?
                .receive(self.reg[send as usize], &self)?;
        } else {
            self.debug();
            err!("No device connected to port: {port}");
        }
        Ok(2)
    }

    pub fn out_reg(&mut self, devices: &Devices, port: Register, send: Register) -> Result<u8> {
        self.out_imm8(devices, self.reg[port as usize], send)?;
        Ok(2)
    }

    pub fn cmp_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: CMP {lhs:#?}, {imm8:02x}", self.pc);

        let diff = (self.reg[lhs as usize] as i16) - (imm8 as i16);
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

    pub fn cmp_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.cmp_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn adc_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: ADC {lhs:#?}, {imm8:02x}", self.pc);

        let f = self.reg[Register::F as usize];
        let cf = (f >> 2) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + Wrapping(imm8) + Wrapping(cf);
        let res = res.0;

        if res < self.reg[lhs as usize] || res < imm8 || res < cf {
            self.reg[Register::F as usize] |= 0b0100;
        }

        self.reg[lhs as usize] = res;
        Ok(2)
    }

    pub fn adc_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.adc_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn sbb_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: SBB {lhs:#?}, {imm8:02x}", self.pc);

        let f = self.reg[Register::F as usize];
        let bf = (f >> 3) & 1;

        let res = Wrapping(self.reg[lhs as usize]) + (Wrapping(!imm8) + Wrapping(1) - Wrapping(bf));
        let res = res.0;

        if res > self.reg[lhs as usize] {
            self.reg[Register::F as usize] = 0b1000;
        }

        self.reg[lhs as usize] = res;
        Ok(2)
    }

    pub fn sbb_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.sbb_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn or_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: OR {lhs:#?}, {imm8:02x}", self.pc);
        self.reg[lhs as usize] |= imm8;
        Ok(2)
    }

    pub fn or_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.or_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn nor_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: NOR {lhs:#?}, {imm8:02x}", self.pc);
        self.reg[lhs as usize] = !(self.reg[lhs as usize] | imm8);
        Ok(2)
    }

    pub fn nor_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.nor_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn and_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        trace!("{:04x}: AND {lhs:#?}, {imm8:02x}", self.pc);
        self.reg[lhs as usize] &= imm8;
        Ok(2)
    }

    pub fn and_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.and_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn set_mb(&mut self, bank: u8) -> Result<u8> {
        trace!("{:04x}: MB {bank:02x}", self.pc);
        self.mem.select(bank)?;
        Ok(2)
    }
}
