use asm::reg::Register;

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
    pub(self) reg: [u8; 8],
    pub pc: u16,
    pub sp: u16,
    pub mem: Mem,
    pub(self) debug: bool,
}

impl CR8 {
    pub fn new(bin: &[u8]) -> Self {
        Self {
            mem: Mem::new(bin),
            reg: [0; 8],
            ..Default::default()
        }
    }

    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn set_stack(mut self, stack: u16) -> Self {
        self.sp = stack;
        self
    }
}
