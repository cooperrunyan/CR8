use anyhow::{anyhow, Result};
use asm::op::Operation;

use crate::cr8::Joinable;
use crate::devices::DeviceID;

use super::Runner;

impl Runner {
    pub fn run(mut self) -> Result<Self> {
        use std::thread;

        loop {
            let (cnt, ticks) = self.cycle()?;
            if !cnt {
                break;
            }

            thread::sleep(self.tickrate * ticks as u32);
        }

        Ok(self)
    }

    pub fn cycle(&mut self) -> Result<(bool, u8)> {
        let mut cr8 = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;

        if let Some(dev) = self.devices.get(DeviceID::SysCtrl) {
            let status = {
                dev.lock()
                    .map_err(|_| anyhow!("Failed to lock mutex"))?
                    .send()?
            };

            if status >> 1 & 1 == 1 {
                cr8.debug();
            }

            if status == 0x01 {
                return Ok((false, 0));
            }
        }

        let pc = cr8.pc;

        let inst = cr8.mem.get(pc)?;

        let op = Self::oper(pc, inst >> 4)?;
        let is_imm = (inst & 0b00001000) == 0b00001000;
        let reg_bits = inst & 0b00000111;

        let b0: u8 = cr8.mem.get(pc + 1).unwrap_or(0);
        let b1: u8 = cr8.mem.get(pc + 2).unwrap_or(0);

        use Operation as O;

        let ticks = match (op, is_imm) {
            (O::LW, true) => cr8.lw_imm16(Self::reg(pc, reg_bits)?, (b0, b1).join()),
            (O::LW, false) => cr8.lw_hl(Self::reg(pc, reg_bits)?),
            (O::SW, true) => cr8.sw_imm16((b0, b1).join(), Self::reg(pc, reg_bits)?),
            (O::SW, false) => cr8.sw_hl(Self::reg(pc, reg_bits)?),
            (O::MOV, true) => cr8.mov_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::MOV, false) => cr8.mov_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::PUSH, true) => cr8.push_imm8(b0),
            (O::PUSH, false) => cr8.push_reg(Self::reg(pc, reg_bits)?),
            (O::POP, _) => cr8.pop(Self::reg(pc, reg_bits)?),
            (O::MB, _) => cr8.set_mb(b0),
            (O::JNZ, true) => cr8.jnz_imm8(b0),
            (O::JNZ, false) => cr8.jnz_reg(Self::reg(pc, reg_bits)?),
            (O::IN, true) => cr8.in_imm8(&self.devices, Self::reg(pc, reg_bits)?, b0),
            (O::IN, false) => {
                cr8.in_reg(&self.devices, Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?)
            }
            (O::OUT, true) => cr8.out_imm8(&self.devices, b0, Self::reg(pc, reg_bits)?),
            (O::OUT, false) => {
                cr8.out_reg(&self.devices, Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?)
            }
            (O::CMP, true) => cr8.cmp_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::CMP, false) => cr8.cmp_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::ADC, true) => cr8.adc_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::ADC, false) => cr8.adc_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::SBB, true) => cr8.sbb_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::SBB, false) => cr8.sbb_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::OR, true) => cr8.or_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::OR, false) => cr8.or_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::NOR, true) => cr8.nor_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::NOR, false) => cr8.nor_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
            (O::AND, true) => cr8.and_imm8(Self::reg(pc, reg_bits)?, b0),
            (O::AND, false) => cr8.and_reg(Self::reg(pc, reg_bits)?, Self::reg(pc, b0)?),
        };

        let ticks = ticks?;
        cr8.pc += ticks as u16;

        Ok((true, ticks))
    }
}
