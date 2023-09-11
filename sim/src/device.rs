use crate::cr8::mem::Mem;
use crate::cr8::CR8;

pub trait Device {
    fn receive(&mut self, reg: &[u8], bank: u8, mem: &Mem, byte: u8);
    fn send(&mut self, reg: &[u8], bank: u8, mem: &Mem) -> u8;
    fn inspect(&self) -> u8;
}

pub use gfx::ID as GFX;
pub use syscontrol::ID as SYS_CONTROL;

mod gfx;
mod syscontrol;

impl CR8 {
    pub fn connect_devices(&mut self) {
        #[cfg(feature = "syscontrol")]
        syscontrol::connect(self);

        #[cfg(feature = "gfx")]
        gfx::connect(self);
    }
}
