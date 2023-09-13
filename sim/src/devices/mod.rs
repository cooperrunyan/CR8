use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use crate::cr8::CR8;

#[cfg(feature = "gfx")]
mod gfx;

#[cfg(feature = "sysctrl")]
mod sysctrl;

#[derive(Default)]
pub struct Devices(Vec<(DeviceID, Arc<Mutex<dyn Device>>)>);

pub trait Device {
    fn attach(&mut self) -> Result<()> {
        Ok(())
    }
    fn tick(&mut self, _cr8: &CR8) -> Result<()> {
        Ok(())
    }
    fn send(&mut self) -> Result<u8> {
        Ok(0)
    }
    fn receive(&mut self, _b: u8, _cr8: &CR8) -> Result<()> {
        Ok(())
    }
    fn inspect(&self) -> usize {
        0
    }
    fn new(cr8: Arc<Mutex<CR8>>) -> Self
    where
        Self: Sized;
}

encodable! {
    pub enum DeviceID {
        else Unknown,
        SysCtrl(0x00),
        Gfx(0x01),
    }
}

impl Devices {
    pub fn connect(&mut self, cr8: Arc<Mutex<CR8>>) -> Result<()> {
        #[cfg(feature = "gfx")]
        {
            self.0.push((
                DeviceID::Gfx,
                Arc::new(Mutex::new(gfx::Gfx::new(cr8.clone()))),
            ));

            let mut cr8 = cr8.lock().map_err(|_| anyhow!("Bad lock"))?;
            cr8.memory.banks.insert(1, [0; 0x4000]);
        }

        #[cfg(feature = "sysctrl")]
        self.0.push((
            DeviceID::SysCtrl,
            Arc::new(Mutex::new(sysctrl::SysCtrl::new(cr8))),
        ));

        Ok(())
    }

    pub fn get(&self, dev: DeviceID) -> Option<Arc<Mutex<dyn Device>>> {
        for (id, d) in self.0.iter() {
            if id == &dev {
                return Some(d.clone());
            }
        }
        None
    }

    pub fn tick(&self, cr8: &CR8) -> Result<()> {
        for (_, d) in self.0.iter() {
            d.clone()
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex to tick devices"))?
                .tick(cr8)?;
        }
        Ok(())
    }
}