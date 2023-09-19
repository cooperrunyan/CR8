use std::io::stdin;

use crate::cr8::mem::Mem;
use crate::cr8::CR8;
use anyhow::Result;
use log::{info, warn};

use super::DeviceSnapshot;

#[derive(Debug, Default)]
pub struct SysCtrl {
    pub state: u8,
    pub debug: bool,
}

encodable! {
    enum SysCtrlSignal {
        else UNKNOWN,
        PING(0x00, "ping"),
        HALT(0x01, "halt"),
        DBG(0x02, "dbg"),
        BRKPT(0x03, "brkpt"),
    }
}

impl SysCtrl {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            ..Default::default()
        }
    }

    pub fn send(&self) -> Result<u8> {
        Ok(self.state)
    }

    pub fn receive(&mut self, byte: u8, cr8: &CR8, mem: &Mem, dev: DeviceSnapshot) -> Result<()> {
        use SysCtrlSignal as SIG;
        match SIG::from(byte) {
            SIG::PING => info!("PONG"),
            SIG::HALT => self.state |= 0b00000001,
            SIG::DBG => cr8.debug(mem, dev),
            SIG::BRKPT => {
                if !self.debug {
                    return Ok(());
                }
                cr8.debug(mem, dev);
                let mut inp = String::new();
                stdin().read_line(&mut inp)?;
            }

            SIG::UNKNOWN => warn!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }
}
