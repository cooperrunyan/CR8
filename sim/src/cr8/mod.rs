use std::sync::RwLock;

use anyhow::{anyhow, Result};

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

#[derive(Debug, Default)]
pub struct CR8 {
    pub reg: [u8; 13],
}

impl CR8 {
    pub fn new() -> Self {
        let mut cr8 = Self::default();
        cr8.set_sp(STACK);
        cr8
    }

    pub fn cycle(&mut self, mem: &RwLock<Mem>, dev: &RwLock<Devices>) -> Result<u8> {
        let pc = self.pc();

        let (inst, b0, b1, b2) = {
            let mem = mem.read().map_err(|_| anyhow!("Read error"))?;
            (
                mem.get(pc)?,
                mem.get(pc + 1).unwrap_or(0),
                mem.get(pc + 2).unwrap_or(0),
                mem.get(pc + 3).unwrap_or(0),
            )
        };

        let ticks = self.delegate(mem, dev, [inst, b0, b1, b2])?;

        self.set_pc(self.pc() + ticks as u16);

        Ok(ticks)
    }

    pub fn pc(&self) -> u16 {
        (
            self.reg[Register::PCL as usize],
            self.reg[Register::PCH as usize],
        )
            .join()
    }

    pub fn set_pc(&mut self, pc: u16) {
        let (l, h) = pc.split();
        self.reg[Register::PCL as usize] = l;
        self.reg[Register::PCH as usize] = h;
    }

    pub fn sp(&self) -> u16 {
        (
            self.reg[Register::SPL as usize],
            self.reg[Register::SPH as usize],
        )
            .join()
    }

    pub fn set_sp(&mut self, sp: u16) {
        let (l, h) = sp.split();
        self.reg[Register::SPL as usize] = l;
        self.reg[Register::SPH as usize] = h;
    }
}
