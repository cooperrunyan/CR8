use crate::cr8::mem::Mem;
use crate::cr8::CR8;
use anyhow::Result;
use log::{debug, warn};

use super::DeviceSnapshot;

#[derive(Debug, Default)]
pub struct SysCtrl {
    pub state: u8,
    peeking: bool,
    peek_low_byte: Option<u8>,
}

encodable! {
    enum SysCtrlSignal {
        else UNKNOWN,
        NOP(0x00, "nop"),
        HALT(0x01, "halt"),
        DBG(0x02, "dbg"),
        PEEK(0x03, "peek"),
    }
}

impl SysCtrl {
    pub fn send(&self) -> Result<u8> {
        Ok(self.state)
    }

    pub fn receive(&mut self, byte: u8, cr8: &CR8, mem: &Mem, dev: DeviceSnapshot) -> Result<()> {
        if self.peeking {
            if self.peek_low_byte.is_none() {
                self.peek_low_byte = Some(byte);
            } else {
                let h = byte;
                let l = self.peek_low_byte.unwrap();

                let addr = ((h as u16) << 8) | l as u16;
                debug!("PEEK {addr}: [{:?}]", mem.get(addr)?);

                self.peeking = false;
                self.peek_low_byte = None;
            }
        }

        use SysCtrlSignal as SIG;
        match SIG::from(byte) {
            SIG::NOP => warn!("sysctrl recieved NOP message"),
            SIG::HALT => self.state |= 0b00000001,
            SIG::PEEK => self.peeking = true,
            SIG::DBG => cr8.debug(mem, dev),

            SIG::UNKNOWN => warn!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }
}
