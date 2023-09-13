use asm::op::Operation;
use asm::reg::Register;

use self::mem::Mem;

use anyhow::{anyhow, bail, Result};

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
    pub reg: [u8; 8],
    pub pcl: u8,
    pub pch: u8,
    pub spl: u8,
    pub sph: u8,
    pub mb: u8,
    pub memory: Mem,
    pub debug: bool,
}

impl CR8 {
    pub fn new() -> Self {
        let mut cr8 = Self::default();

        cr8.set_sp(STACK.split());

        cr8
    }
    pub fn load(&mut self, bin: &[u8]) {
        for (i, byte) in bin.iter().enumerate() {
            self.memory.rom[i] = *byte;
        }
    }
}
