use std::collections::HashMap;

#[derive(Debug)]
pub struct Mem {
    pub rom: [u8; 0x8000],
    pub builtin_ram: [u8; 0x8000],
    pub banks: HashMap<u8, [u8; 0x4000]>,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            rom: [0; 0x8000],
            builtin_ram: [0; 0x8000],
            banks: HashMap::new(),
        }
    }
}

impl Mem {
    pub fn get(&self, bank: u8, addr: u16) -> u8 {
        if addr & 0b10000000_00000000 == 0 {
            return self.rom[(addr & 0b01111111_11111111) as usize];
        }
        if bank != 0 {
            return match self.banks.get(&bank) {
                Some(bank) => bank[(addr & 0b00111111_11111111) as usize],
                None => panic!("Bank {bank} does not exist"),
            };
        }
        return self.builtin_ram[(addr & 0b01111111_11111111) as usize];
    }

    pub fn set(&mut self, bank: u8, addr: u16, value: u8) {
        if addr & 0b10000000_00000000 == 0 {
            return;
        }
        if bank != 0 {
            match self.banks.get_mut(&bank) {
                Some(bank) => bank[(addr & 0b00111111_11111111) as usize] = value,
                None => panic!("Bank {bank} does not exist"),
            }
        } else {
            self.builtin_ram[(addr & 0b01111111_11111111) as usize] = value;
        }
    }
}
