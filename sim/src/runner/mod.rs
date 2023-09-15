use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};

use super::devices::Devices;
use crate::cr8::{CR8, STACK};
use crate::devices::DeviceID;

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
    pub fn new(bin: &[u8], tickrate: Duration) -> Self {
        let cr8 = Arc::new(Mutex::new(CR8::new(bin).set_stack(STACK)));
        let mut devices = Devices::default();
        devices.connect(cr8.clone());

        Self {
            tickrate,
            devices,
            cr8,
        }
    }

    pub fn debug(&self) -> Result<()> {
        self.cr8
            .lock()
            .map_err(|_| anyhow!("Failed to get a lock"))?
            .debug();
        Ok(())
    }

    pub fn cycle(&mut self) -> Result<u8> {
        let mut cr8 = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;

        if let Some(dev) = self.devices.get(DeviceID::SysCtrl) {
            let status = {
                dev.lock()
                    .map_err(|_| anyhow!("Failed to lock mutex"))?
                    .send()?
            };

            if status >> 1 & 1 == 1 {
                cr8.debug();
            }

            if status == 0x01 {
                exit(0);
            }
        }

        let ticks = cr8
            .cycle(&self.devices)
            .context(format!("Cycle failed at {:#06x?}", cr8.pc))?;

        Ok(ticks)
    }
}
