use crate::cr8::CR8;
use anyhow::Result;
use std::sync::{Arc, Mutex};

use super::Device;

#[derive(Debug, Default)]
pub struct Gfx {
    pub tick: u16,
    pub byte: u8,
}

impl Device for Gfx {
    fn send(&mut self) -> Result<u8> {
        Ok(self.byte)
    }

    fn inspect(&self) -> usize {
        self.tick as usize
    }

    fn tick(&mut self, cr8: &CR8) -> Result<()> {
        self.tick = self.tick.wrapping_add(1);
        if self.tick > 16383 {
            self.tick = 0;
        }
        self.byte = cr8.memory.get(1, self.tick | 0b10000000_00000000);

        Ok(())
    }

    fn new(_: Arc<Mutex<CR8>>) -> Self
    where
        Self: Sized,
    {
        Self {
            ..Default::default()
        }
    }
}
