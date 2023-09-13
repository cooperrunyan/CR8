use std::num::Wrapping;

use anyhow::{anyhow, Result};
use asm::reg::Register;

use crate::cr8::{Joinable, Splittable};
use crate::devices::{DeviceID, Devices};

use super::{CR8, STACK, STACK_END};
macro_rules! cr8debug {
    ($self:ident, $msg:expr $(,$args:expr)*) => {
        if $self.debug {
            println!($msg $(,$args)*);
        }
    }
}

impl CR8 {
    pub fn lw_imm16(&mut self, to: Register, i: (u8, u8)) -> Result<u8> {
        let addr = i.join();
        cr8debug!(self, "LW {} {to:#?}, {addr:#?}", self.mb);
        self.reg[to as usize] = self.memory.get(self.mb, addr);
        Ok(3)
    }

    pub fn lw_hl(&mut self, to: Register) -> Result<u8> {
        let addr = self.hl().join();
        cr8debug!(self, "LW {} {to:#?}, {}", self.mb, addr);
        self.reg[to as usize] = self.memory.get(self.mb, addr);
        Ok(1)
    }

    pub fn sw_hl(&mut self, from: Register) -> Result<u8> {
        cr8debug!(self, "SW {} {from:#?}, {}", self.mb, self.hl().join());
        self.memory
            .set(self.mb, self.hl().join(), self.reg[from as usize]);
        Ok(1)
    }

    pub fn sw_imm16(&mut self, i: (u8, u8), from: Register) -> Result<u8> {
        cr8debug!(self, "SW {} {from:#?}, {}", self.mb, i.join());
        self.memory
            .set(self.mb, i.join(), self.reg[from as usize].clone());
        Ok(3)
    }

    pub fn mov_reg(&mut self, to: Register, from: Register) -> Result<u8> {
        cr8debug!(self, "MOV {to:#?}, {from:#?}");

        self.reg[to as usize] = self.reg[from as usize];
        Ok(2)
    }

    pub fn mov_imm8(&mut self, to: Register, imm8: u8) -> Result<u8> {
        cr8debug!(self, "MOV {to:#?}, {imm8:#?}");
        self.reg[to as usize] = imm8;
        Ok(2)
    }

    pub fn push_imm8(&mut self, imm8: u8) -> Result<u8> {
        let sptr = self.sp().join();

        if sptr >= STACK_END {
            panic!("Stack overflow");
        }

        self.set_sp((sptr + 1).split());
        self.memory.set(self.mb, self.sp().join(), imm8);

        cr8debug!(
            self,
            "PUSHED: [{}] {}",
            self.sp().join() as i128 - STACK as i128,
            imm8
        );
        Ok(2)
    }

    pub fn push_reg(&mut self, reg: Register) -> Result<u8> {
        self.push_imm8(self.reg[reg as usize])?;
        Ok(1)
    }

    pub fn pop(&mut self, reg: Register) -> Result<u8> {
        let sptr = self.sp().join();

        if sptr < STACK {
            panic!("Cannot pop empty stack");
        }

        self.reg[reg as usize] = self.memory.get(self.mb, sptr);
        self.memory.set(self.mb, sptr, 0);

        cr8debug!(
            self,
            "POPPED: [{}] {}",
            sptr - STACK,
            self.reg[reg as usize]
        );

        self.set_sp((sptr - 1).split());
        Ok(1)
    }

    pub fn jnz_imm8(&mut self, imm8: u8) -> Result<u8> {
        if imm8 == 0 {
            cr8debug!(self, "No JNZ");
            return Ok(2);
        }

        self.pcl = self.reg[Register::L as usize];
        self.pch = self.reg[Register::H as usize];

        cr8debug!(self, "JNZ to {}", self.pc().join());
        Ok(0)
    }

    pub fn jnz_reg(&mut self, reg: Register) -> Result<u8> {
        let v = self.reg[reg as usize];
        self.jnz_imm8(self.reg[reg as usize])?;
        if v == 0 {
            return Ok(1);
        }
        return Ok(0);
    }

    pub fn in_imm8(&mut self, devices: &Devices, into: Register, port: u8) -> Result<u8> {
        cr8debug!(self, "IN {into:#?}, {port:#?}");

        if let Some(dev) = devices.get(DeviceID::from(port)) {
            self.reg[into as usize] = dev
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?
                .send()?;
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
        Ok(2)
    }

    pub fn in_reg(&mut self, devices: &Devices, into: Register, port: Register) -> Result<u8> {
        self.in_imm8(devices, into, self.reg[port as usize])?;
        Ok(2)
    }

    pub fn out_imm8(&mut self, devices: &Devices, port: u8, send: Register) -> Result<u8> {
        cr8debug!(self, "OUT {send:#?}, {port:#?}");
        if let Some(dev) = devices.get(DeviceID::from(port)) {
            dev.lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?
                .receive(self.reg[send as usize], &self)?;
        } else {
            self.debug();
            panic!("No device connected to port: {port}");
        }
        Ok(2)
    }

    pub fn out_reg(&mut self, devices: &Devices, port: Register, send: Register) -> Result<u8> {
        self.out_imm8(devices, self.reg[port as usize], send)?;
        Ok(2)
    }

    pub fn cmp_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
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
        Ok(2)
    }

    pub fn cmp_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.cmp_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn adc_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        cr8debug!(self, "ADC {lhs:#?}, {imm8:#?}");

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
        cr8debug!(self, "SBB {lhs:#?}, {imm8:#?}");

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
        cr8debug!(self, "OR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] |= imm8;
        Ok(2)
    }

    pub fn or_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.or_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn nor_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        cr8debug!(self, "NOR {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] = !(self.reg[lhs as usize] | imm8);
        Ok(2)
    }

    pub fn nor_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.nor_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn and_imm8(&mut self, lhs: Register, imm8: u8) -> Result<u8> {
        cr8debug!(self, "AND {lhs:#?}, {imm8:#?}");
        self.reg[lhs as usize] &= imm8;
        Ok(2)
    }

    pub fn and_reg(&mut self, lhs: Register, reg: Register) -> Result<u8> {
        self.and_imm8(lhs, self.reg[reg as usize])?;
        Ok(2)
    }

    pub fn set_mb(&mut self, bank: u8) -> Result<u8> {
        cr8debug!(self, "MB {bank:#?}");
        self.mb = bank;
        Ok(2)
    }
}
