use std::io::stdin;

use crate::cr8::mem::Mem;
use crate::cr8::CR8;
use anyhow::Result;
use log::{info, warn};

use super::DeviceSnapshot;

/// Communicates with the [crate::cr8::CR8].
/// When it receives a byte, it will parse it as a [SysCtrlSignal].
#[derive(Debug, Default)]
pub struct SysCtrl {
    pub state: u8,
    pub debug: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SysCtrlSignal {
    /// Ping the controller
    Ping,

    /// Stop the clock
    Halt,

    /// Print [crate::cr8::CR8] state data
    Debug,

    /// Pause the clock until stdin receives a line
    Breakpoint,
}

impl TryFrom<u8> for SysCtrlSignal {
    type Error = ();
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use SysCtrlSignal as SIG;
        Ok(match value {
            0x00 => SIG::Ping,
            0x01 => SIG::Halt,
            0x02 => SIG::Debug,
            0x03 => SIG::Breakpoint,
            _ => Err(())?,
        })
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
        match SIG::try_from(byte) {
            Ok(s) => match s {
                SIG::Ping => info!("PONG"),
                SIG::Halt => self.state |= 0b00000001,
                SIG::Debug => cr8.debug(mem, dev),
                SIG::Breakpoint => {
                    if !self.debug {
                        return Ok(());
                    }
                    cr8.debug(mem, dev);
                    let mut inp = String::new();
                    stdin().read_line(&mut inp)?;
                }
            },
            Err(_) => warn!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }
}
