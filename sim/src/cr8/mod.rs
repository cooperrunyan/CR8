use std::sync::RwLock;

use anyhow::{anyhow, Result};

use asm::reg::Register;

use crate::devices::Devices;

use self::mem::Mem;

pub mod mem;

mod debug;
mod inst;

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

/// The center of everything.
///
/// Holds 13 (1-byte) registers and has the ability to perform
/// an [asm::op::Operation] on them.
#[derive(Debug, Default)]
pub struct CR8 {
    pub reg: [u8; 9],
    pub pc: u16,
    pub sp: u16,
}

impl CR8 {
    pub fn new() -> Self {
        Self {
            sp: STACK,
            ..Default::default()
        }
    }

    /// Do an [asm::op::Operation]
    pub fn cycle(&mut self, mem: &RwLock<Mem>, dev: &RwLock<Devices>) -> Result<u8> {
        let (inst, b0, b1, b2) = {
            let mem = mem.read().map_err(|_| anyhow!("Read error"))?;
            (
                mem.get(self.pc)?,
                mem.get(self.pc + 1).unwrap_or(0),
                mem.get(self.pc + 2).unwrap_or(0),
                mem.get(self.pc + 3).unwrap_or(0),
            )
        };

        let ticks = self.delegate(mem, dev, [inst, b0, b1, b2])?;

        self.pc += ticks as u16;

        Ok(ticks)
    }

    /// Get XY by turning X and Y into a single u16
    pub fn xy(&self) -> u16 {
        let x = self.reg[Register::X as usize];
        let y = self.reg[Register::Y as usize];

        (x, y).join()
    }
}
