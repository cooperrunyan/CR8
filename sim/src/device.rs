use asm::reg::Register;

use crate::cr8::{Mem, SIGDBG, SIGHALT, SIGNOP, SIGPEEK};

pub trait Device {
    fn receive(&mut self, reg: &[u8], bank: u8, mem: &Mem, byte: u8);
    fn send(&mut self, reg: &[u8], bank: u8, mem: &Mem) -> u8;
    fn inspect(&self) -> u8;
}

#[derive(Default, Debug)]
pub struct Control {
    pub state: u8,
    peeking: bool,
    peek_low_byte: Option<u8>,
}

impl Device for Control {
    fn receive(&mut self, reg: &[u8], bank: u8, mem: &Mem, byte: u8) {
        if self.peeking {
            if self.peek_low_byte.is_none() {
                self.peek_low_byte = Some(byte);
            } else {
                let h = byte;
                let l = self.peek_low_byte.unwrap();

                let addr = ((h as u16) << 8) | l as u16;
                println!("PEEK {addr}: [{}]", mem.get(bank, addr));

                self.peeking = false;
                self.peek_low_byte = None;
            }
        }

        match byte {
            SIGNOP => println!("Control recieved NOP message"),
            SIGHALT => {
                self.state |= 0b00000001;
            }
            SIGPEEK => {
                self.peeking = true;
            }
            SIGDBG => {
                println!("A: {}", reg[Register::A as usize]);
                println!("B: {}", reg[Register::B as usize]);
                println!("C: {}", reg[Register::C as usize]);
                println!("D: {}", reg[Register::D as usize]);
                println!("Z: {}", reg[Register::Z as usize]);
            }

            _ => {}
        }
    }

    fn send(&mut self, _reg: &[u8], _bank: u8, _mem: &Mem) -> u8 {
        self.state
    }

    fn inspect(&self) -> u8 {
        self.state
    }
}
