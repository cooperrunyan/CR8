use crate::cr8::CR8;
use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use super::Device;

#[derive(Debug, Default)]
pub struct SysCtrl {
    cr8: Arc<Mutex<CR8>>,

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

    fn receive(&mut self, byte: u8) -> Result<()> {
        if self.peeking {
            if self.peek_low_byte.is_none() {
                self.peek_low_byte = Some(byte);
            } else {
                let h = byte;
                let l = self.peek_low_byte.unwrap();

                let addr = ((h as u16) << 8) | l as u16;
                let cr8 = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;
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
            SIG::DBG => self.state |= 0b00000010,

            SIG::UNKNOWN => println!("sysctrl recieved unknown {byte:#?} message"),
        };
        Ok(())
    }

    fn new(cr8: Arc<Mutex<CR8>>) -> Self
    where
        Self: Sized,
    {
        Self {
            cr8,
            ..Default::default()
        }
    }
    fn attach(&mut self) -> Result<()> {
        Ok(())
    }
    fn tick(&mut self) -> Result<()> {
        Ok(())
    }
}
