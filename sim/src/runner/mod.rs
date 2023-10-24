use std::sync::RwLock;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};

use super::devices::Devices;
use crate::cr8::mem::Mem;
use crate::cr8::CR8;

mod config;

/// Wraps around a [CR8].
///
/// ## Purpose
///
/// - Tells the [CR8] to tick. On each tick, tells it what to do.
/// - Provides the [CR8] with memory to read from or write to.
pub struct Runner {
    pub mem: RwLock<Mem>,
    pub cr8: RwLock<CR8>,
    pub devices: RwLock<Devices>,
    pub tickrate: Duration,
}

impl Runner {
    /// Create a new runner and load a byte slice to ROM
    pub fn new(bin: &[u8], tickrate: Duration, debug: bool) -> Self {
        let mem = RwLock::new(Mem::new(bin));
        let devices = RwLock::new(Devices::new(debug));
        let cr8 = RwLock::new(CR8::new());

        Self {
            tickrate,
            devices,
            cr8,
            mem,
        }
    }

    pub fn debug(&self) -> Result<()> {
        let mem = self.mem.read().map_err(|_| anyhow!("Poisoned"))?;
        let dev = self.devices.read().map_err(|_| anyhow!("Poisoned"))?;
        self.cr8.read().unwrap().debug(&mem, dev.snapshot());
        Ok(())
    }

    /// Tick the [CR8]
    pub fn cycle(&mut self) -> Result<(u8, bool)> {
        {
            let status = {
                let dev = self.devices.read().map_err(|_| anyhow!("Poisoned"))?;
                dev.sysctrl.state
            };

            // if status >> 1 & 1 == 1 {
            //     self.debug()?;
            //     if self.debug {
            //         let mut inp = String::new();
            //         stdin().read_line(&mut inp)?;
            //         if &inp == "q" {
            //             return Ok((0, false));
            //         }
            //     }
            // }

            if status == 0x01 {
                return Ok((0, false));
            }
        }
        let mut cr8 = self.cr8.write().unwrap();

        let ticks = cr8
            .cycle(&self.mem, &self.devices)
            .context(format!("Cycle failed at {:#06x?}", cr8.pc))?;

        Ok((ticks, true))
    }
}
