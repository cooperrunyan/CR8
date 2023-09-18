use std::sync::RwLock;

use anyhow::{anyhow, bail, Result};
use asm::op::Operation;
use asm::reg::Register;

use crate::devices::Devices;

use self::mem::Mem;

pub mod mem;

mod debug;
mod inst;
mod probe;

pub const STACK: u16 = 0xFC00;
pub const STACK_END: u16 = 0xFEFF;

pub trait Splittable {
    fn split(&self) -> (u8, u8);
}

pub trait Joinable {
    fn join(&self) -> u16;
}

impl Splittable for u16 {
    fn split(&self) -> (u8, u8) {
        ((*self as u8), (*self >> 8) as u8)
    }
}

impl Joinable for (u8, u8) {
    fn join(&self) -> u16 {
        let (l, h) = *self;
        ((h as u16) << 8) | (l as u16)
    }
}

#[derive(Debug)]
pub struct CR8 {
    pub reg: [u8; 8],
    pub pc: u16,
    pub sp: u16,
}

impl CR8 {
    pub fn new() -> Self {
        Self {
            reg: [0; 8],
            pc: 0,
            sp: STACK,
        }
    }

    pub fn cycle(&mut self, mem: &RwLock<Mem>, dev: &RwLock<Devices>) -> Result<u8> {
        let pc = self.pc;

        let (inst, b0, b1) = {
            let mem = mem.read().map_err(|_| anyhow!("Read error"))?;
            (
                mem.get(pc)?,
                mem.get(pc + 1).unwrap_or(0),
                mem.get(pc + 2).unwrap_or(0),
            )
        };

        let op = oper(pc, inst >> 4)?;
        let is_imm = (inst & 0b00001000) == 0b00001000;
        let reg_bits = inst & 0b00000111;

        use Operation as O;

        let ticks = match (op, is_imm) {
            (O::LW, true) => self.lw_imm16(mem, reg(pc, reg_bits)?, (b0, b1).join()),
            (O::LW, false) => self.lw_hl(mem, reg(pc, reg_bits)?),
            (O::SW, true) => self.sw_imm16(mem, (b0, b1).join(), reg(pc, reg_bits)?),
            (O::SW, false) => self.sw_hl(mem, reg(pc, reg_bits)?),
            (O::MOV, true) => self.mov_imm8(reg(pc, reg_bits)?, b0),
            (O::MOV, false) => self.mov_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::PUSH, true) => self.push_imm8(mem, b0),
            (O::PUSH, false) => self.push_reg(mem, reg(pc, reg_bits)?),
            (O::POP, _) => self.pop(mem, reg(pc, reg_bits)?),
            (O::MB, _) => self.set_mb(mem, b0),
            (O::JNZ, true) => self.jnz_imm8(b0),
            (O::JNZ, false) => self.jnz_reg(reg(pc, reg_bits)?),
            (O::IN, true) => self.in_imm8(dev, reg(pc, reg_bits)?, b0),
            (O::IN, false) => self.in_reg(dev, reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::OUT, true) => self.out_imm8(mem, dev, b0, reg(pc, reg_bits)?),
            (O::OUT, false) => self.out_reg(mem, dev, reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::CMP, true) => self.cmp_imm8(reg(pc, reg_bits)?, b0),
            (O::CMP, false) => self.cmp_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::ADC, true) => self.adc_imm8(reg(pc, reg_bits)?, b0),
            (O::ADC, false) => self.adc_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::SBB, true) => self.sbb_imm8(reg(pc, reg_bits)?, b0),
            (O::SBB, false) => self.sbb_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::OR, true) => self.or_imm8(reg(pc, reg_bits)?, b0),
            (O::OR, false) => self.or_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::NOR, true) => self.nor_imm8(reg(pc, reg_bits)?, b0),
            (O::NOR, false) => self.nor_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
            (O::AND, true) => self.and_imm8(reg(pc, reg_bits)?, b0),
            (O::AND, false) => self.and_reg(reg(pc, reg_bits)?, reg(pc, b0)?),
        };

        let ticks = ticks?;

        self.pc += ticks as u16;

        Ok(ticks)
    }
}

fn reg(pc: u16, byte: u8) -> Result<Register> {
    match Register::try_from(byte) {
        Ok(r) => Ok(r),
        Err(_) => bail!("Invalid register: {byte} at {pc}"),
    }
}

fn oper(pc: u16, byte: u8) -> Result<Operation> {
    match Operation::try_from(byte) {
        Ok(r) => Ok(r),
        Err(_) => bail!("Invalid operation: {byte} at {pc}"),
    }
}
