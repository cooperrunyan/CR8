use anyhow::{bail, Result};
use std::fmt::Debug;

use crate::cr8::mem::Mem;
use crate::cr8::CR8;

use self::sysctrl::SysCtrl;

pub mod sysctrl;

#[cfg(feature = "keyboard")]
pub mod keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceId {
    SysControl,
    Keyboard,
}

impl TryFrom<u8> for DeviceId {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::SysControl),
            0x01 => Ok(Self::Keyboard),
            _ => Err(()),
        }
    }
}

impl Into<u8> for DeviceId {
    fn into(self) -> u8 {
        match self {
            Self::SysControl => 0x00,
            Self::Keyboard => 0x01,
        }
    }
}

#[derive(Debug, Default)]
pub struct Devices {
    pub sysctrl: SysCtrl,
    #[cfg(feature = "keyboard")]
    pub keyboard: keyboard::Keyboard,
}

#[derive(Debug)]
pub struct DeviceSnapshot {
    pub sysctrl: u8,
    #[cfg(feature = "keyboard")]
    pub keyboard: u8,
}

impl Devices {
    pub fn new(debug: bool) -> Self {
        Self {
            sysctrl: SysCtrl::new(debug),
            ..Default::default()
        }
    }

    pub fn send(
        &mut self,
        cr8: &CR8,
        mem: &Mem,
        to: impl TryInto<DeviceId> + Debug + Copy,
        byte: u8,
    ) -> Result<()> {
        #[allow(unreachable_patterns)]
        match to.try_into() {
            Ok(DeviceId::SysControl) => self.sysctrl.receive(byte, cr8, mem, self.snapshot()),

            #[cfg(feature = "keyboard")]
            Ok(DeviceId::Keyboard) => Ok(()),

            Ok(d) => bail!("Device {d:?} not connected"),
            Err(_) => bail!("Unknown device: {to:?}"),
        }
    }

    pub fn receive(&mut self, to: impl TryInto<DeviceId> + Debug + Copy) -> Result<u8> {
        #[allow(unreachable_patterns)]
        match to.try_into() {
            Ok(DeviceId::SysControl) => self.sysctrl.send(),

            #[cfg(feature = "keyboard")]
            Ok(DeviceId::Keyboard) => Ok(self.keyboard.flush()),

            Ok(d) => bail!("Device {d:?} not connected"),
            Err(_) => bail!("Unknown device: {to:?}"),
        }
    }

    pub fn snapshot(&self) -> DeviceSnapshot {
        DeviceSnapshot {
            sysctrl: self.sysctrl.state,
            #[cfg(feature = "keyboard")]
            keyboard: self.keyboard.0,
        }
    }
}
