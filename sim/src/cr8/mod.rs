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
    pub reg: [u8; 13],
}

impl CR8 {
    pub fn new() -> Self {
        let mut cr8 = Self::default();
        cr8.set_sp(STACK);
        cr8
    }

    /// Do an [asm::op::Operation]
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

    /// Get Program Counter by turning PCL and PCH into a single u16
    pub fn pc(&self) -> u16 {
        (
            self.reg[Register::PCL as usize],
            self.reg[Register::PCH as usize],
        )
            .join()
    }

    /// Set PCL and PCH from a u16
    pub fn set_pc(&mut self, pc: u16) {
        let (l, h) = pc.split();
        self.reg[Register::PCL as usize] = l;
        self.reg[Register::PCH as usize] = h;
    }

    /// Get Stack Pointer by turning PCL and PCH into a single u16
    pub fn sp(&self) -> u16 {
        (
            self.reg[Register::SPL as usize],
            self.reg[Register::SPH as usize],
        )
            .join()
    }

    /// Set SPL and SPH from a u16
    pub fn set_sp(&mut self, sp: u16) {
        let (l, h) = sp.split();
        self.reg[Register::SPL as usize] = l;
        self.reg[Register::SPH as usize] = h;
    }

    /// Get HL by turning L and H into a single u16
    pub fn hl(&self) -> u16 {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        (l, h).join()
    }
}
