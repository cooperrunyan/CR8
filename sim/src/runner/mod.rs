use std::process::exit;
use std::sync::RwLock;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};

use super::devices::Devices;
use crate::cr8::mem::Mem;
use crate::cr8::CR8;

mod config;

pub struct Runner {
    pub mem: RwLock<Mem>,
    pub cr8: RwLock<CR8>,
    pub devices: RwLock<Devices>,
    pub tickrate: Duration,
}

impl Runner {
    pub fn new(bin: &[u8], tickrate: Duration) -> Self {
        let mem = RwLock::new(Mem::new(bin));
        let devices = RwLock::new(Devices::default());
        let cr8 = RwLock::new(CR8::new());

        Self {
            tickrate,
            devices,
            cr8,
            mem,
        }
    }

    pub fn debug(&self) -> Result<()> {
        let mem = self.mem.read().map_err(|_| anyhow!("Mutex"))?;
        self.cr8.read().unwrap().debug(&mem);
        Ok(())
    }

    pub fn cycle(&mut self) -> Result<u8> {
        {
            let dev = self.devices.read().map_err(|_| anyhow!("Mutex poisoned"))?;

            let status = dev.sysctrl.state;

            if status >> 1 & 1 == 1 {
                self.debug()?;
            }

            if status == 0x01 {
                exit(0);
            }
        }
        let mut cr8 = self.cr8.write().unwrap();

        let ticks = cr8
            .cycle(&self.mem, &self.devices)
            .context(format!("Cycle failed at {:#06x?}", cr8.pc))?;

        Ok(ticks)
    }
}
