use crate::cr8::CR8;
use anyhow::Result;
use std::sync::{Arc, Mutex};

use super::Device;

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
        PEEK(0x02, "peek"),
        DBG(0x03, "dbg"),
    }
}

impl Device for SysCtrl {
    fn send(&mut self) -> Result<u8> {
        Ok(self.state)
    }

    fn receive(&mut self, byte: u8, cr8: &CR8) -> Result<()> {
        if self.peeking {
            if self.peek_low_byte.is_none() {
                self.peek_low_byte = Some(byte);
            } else {
                let h = byte;
                let l = self.peek_low_byte.unwrap();

                let addr = ((h as u16) << 8) | l as u16;
                println!("PEEK {addr}: [{}]", cr8.memory.get(cr8.mb, addr));

                self.peeking = false;
                self.peek_low_byte = None;
            }
        }

        use SysCtrlSignal as SIG;
        match SIG::from(byte) {
            SIG::NOP => println!("sysctrl recieved NOP message"),
            SIG::HALT => self.state |= 0b00000001,
            SIG::PEEK => self.peeking = true,
            SIG::DBG => cr8.debug(),

            SIG::UNKNOWN => println!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }

    fn new(_cr8: Arc<Mutex<CR8>>) -> Self
    where
        Self: Sized,
    {
        Self {
            ..Default::default()
        }
    }
}
