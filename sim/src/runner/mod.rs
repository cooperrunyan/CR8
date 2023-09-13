use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, bail, Result};

use asm::{op::Operation, reg::Register};

use super::devices::Devices;
use crate::cr8::{CR8, STACK};

mod config;

#[cfg(not(feature = "gfx"))]
mod base;

#[cfg(feature = "gfx")]
mod gfx;

#[derive(Default)]
pub struct Runner {
    cr8: Arc<Mutex<CR8>>,
    devices: Devices,
    tickrate: Duration,
}

impl Runner {
    pub fn new(tickrate: Duration, debug: bool) -> Self {
        let mut cr8 = CR8::new();
        cr8.sp = STACK;
        cr8.debug = debug;

        #[cfg(feature = "gfx")]
        cr8.memory.banks.insert(1, [0; 0x4000]);

        Self {
            tickrate,
            devices: Devices::default(),
            cr8: Arc::new(Mutex::new(cr8)),
        }
    }

    pub fn debug(&self) -> Result<()> {
        self.cr8
            .lock()
            .map_err(|_| anyhow!("Failed to get a lock"))?
            .debug();
        Ok(())
    }

    pub fn load(&mut self, bin: &[u8]) -> Result<()> {
        {
            let mut c = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;
            c.memory.rom[..bin.len()].copy_from_slice(bin);
        }

        self.devices.connect(self.cr8.clone())?;

        Ok(())
    }

    pub(crate) fn reg(pc: u16, byte: u8) -> Result<Register> {
        match Register::try_from(byte) {
            Ok(r) => Ok(r),
            Err(_) => bail!("Invalid register: {byte} at {pc}"),
        }
    }

    pub(crate) fn oper(pc: u16, byte: u8) -> Result<Operation> {
        match Operation::try_from(byte) {
            Ok(r) => Ok(r),
            Err(_) => bail!("Invalid operation: {byte} at {pc}"),
        }
    }
}
