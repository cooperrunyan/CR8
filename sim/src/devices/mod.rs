use anyhow::{bail, Result};
use std::fmt::Debug;

use crate::cr8::mem::Mem;
use crate::cr8::CR8;

use self::sysctrl::SysCtrl;

mod sysctrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceId {
    SysControl,
}

impl TryFrom<u8> for DeviceId {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::SysControl),
            _ => Err(()),
        }
    }
}

impl Into<u8> for DeviceId {
    fn into(self) -> u8 {
        match self {
            Self::SysControl => 0x00,
        }
    }
}

#[derive(Debug, Default)]
pub struct Devices {
    pub sysctrl: SysCtrl,
}

impl Devices {
    pub fn send(
        &mut self,
        cr8: &CR8,
        mem: &Mem,
        to: impl TryInto<DeviceId> + Debug + Copy,
        byte: u8,
    ) -> Result<()> {
        match to.try_into() {
            Ok(DeviceId::SysControl) => self.sysctrl.receive(byte, cr8, mem),
            Err(_) => bail!("Unknown device: {to:?}"),
        }
    }

    pub fn receive(&self, to: impl TryInto<DeviceId> + Debug + Copy) -> Result<u8> {
        match to.try_into() {
            Ok(DeviceId::SysControl) => self.sysctrl.send(),
            Err(_) => bail!("Unknown device: {to:?}"),
        }
    }
}
