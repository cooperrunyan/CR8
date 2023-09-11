use crate::cr8::mem::Mem;
use crate::cr8::CR8;

use super::Device;

#[derive(Default, Debug)]
pub struct Gfx {
    busy: u8,
}

pub const ID: u8 = 0x01;
pub const VRAM_BANK: u8 = 0x01;

impl Device for Gfx {
    fn receive(&mut self, _reg: &[u8], _bank: u8, mem: &Mem, _byte: u8) {
        self.busy = 1;
        let vram = mem.banks.get(&VRAM_BANK).unwrap();
        let mut str = String::new();
        macro_rules! push {
            ($i:expr) => {
                if $i == 1 {
                    str.push('#');
                } else {
                    str.push(' ')
                }
            };
        }
        for (i, byte) in vram.iter().enumerate() {
            if i % 16 == 0 {
                str.push('\n');
            }
            push!(byte >> 7 & 1);
            push!(byte >> 6 & 1);
            push!(byte >> 5 & 1);
            push!(byte >> 4 & 1);
            push!(byte >> 3 & 1);
            push!(byte >> 2 & 1);
            push!(byte >> 1 & 1);
            push!(byte & 1);
            if i == 16 * 8 {
                break;
            }
        }
        println!("{}", str);
        self.busy = 0;
    }

    fn send(&mut self, _reg: &[u8], _bank: u8, _mem: &Mem) -> u8 {
        self.busy
    }

    fn inspect(&self) -> u8 {
        0
    }
}

pub fn connect(cr8: &mut CR8) {
    cr8.memory.banks.insert(VRAM_BANK, [0; 0x4000]);
    cr8.dev_add(ID, Box::<Gfx>::default());
}
