use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use crate::cr8::CR8;

#[cfg(feature = "sysctrl")]
mod sysctrl;

#[derive(Default)]
pub struct Devices(Vec<(DeviceID, Arc<Mutex<dyn Device>>)>);

pub trait Device {
    fn attach(&mut self) -> Result<()>;
    fn tick(&mut self) -> Result<()>;
    fn send(&mut self) -> Result<u8>;
    fn receive(&mut self, b: u8) -> Result<()>;
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
    pub fn connect(&mut self, cr8: Arc<Mutex<CR8>>) {
        #[cfg(feature = "sysctrl")]
        self.0.push((
            DeviceID::SysCtrl,
            Arc::new(Mutex::new(sysctrl::SysCtrl::new(cr8))),
        ));
    }

    pub fn get(&self, dev: DeviceID) -> Option<Arc<Mutex<dyn Device>>> {
        for (id, d) in self.0.iter() {
            if id == &dev {
                return Some(d.clone());
            }
        }
        None
    }

    pub fn tick(&self) -> Result<()> {
        for (_, d) in self.0.iter() {
            d.clone()
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex to tick devices"))?
                .tick()?;
        }
        Ok(())
    }
}
