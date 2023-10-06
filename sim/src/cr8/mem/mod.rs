mod bank;

use anyhow::{bail, Result};
use std::fmt::Debug;

use bank::{mask, BankCollection, BankId};

pub(self) const ROM_START: usize = 0x0000;
pub(self) const ROM_LEN: usize = 0x8000;

pub(self) const RAM_LEN: usize = 0x8000;
pub const BANK_LEN: usize = 0x4000;

pub(self) const ROM_END: usize = ROM_START + ROM_LEN - 1;
pub const RAM_START: usize = ROM_END + 1;
pub(self) const BANK_END: usize = RAM_LEN + BANK_LEN - 1;

pub(self) const BANK_MASK: usize = BANK_LEN - 1;
pub(self) const RAM_MASK: usize = RAM_LEN - 1;

#[derive(Debug)]
pub struct Mem {
    selected: BankId,
    rom: [u8; ROM_LEN],
    builtin_ram: [u8; RAM_LEN],
    pub banks: BankCollection,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            rom: [0; ROM_LEN],
            builtin_ram: [0; RAM_LEN],
            banks: BankCollection::default(),
            selected: BankId::Builtin,
        }
    }
}

impl Mem {
    pub fn new(bin: &[u8]) -> Self {
        let mut rom = [0; ROM_LEN];
        rom[..bin.len()].copy_from_slice(bin);

        Self {
            rom,
            ..Default::default()
        }
    }

    pub fn select(&mut self, id: impl TryInto<BankId> + Debug + Clone) -> Result<()> {
        self.selected = BankId::check(id)?;
        Ok(())
    }

    pub fn get(&self, addr: impl Into<usize>) -> Result<u8> {
        let addr: usize = addr.into();
        if addr <= ROM_END {
            Ok(self.rom[mask(addr)])
        } else {
            if addr > BANK_END {
                return Ok(self.builtin_ram[mask(addr)]);
            }

            use BankId as B;
            match self.selected {
                B::Builtin => Ok(self.builtin_ram[mask(addr)]),
                oth => match self.banks.get(oth.clone()).map(|b| b.unwrap())?.get(addr) {
                    Some(x) => Ok(x),
                    None => bail!("Address {addr:#06x?} not found in {oth:#?}.",),
                },
            }
        }
    }

    #[cfg(feature = "gfx")]
    pub fn get_vram(&self, addr: impl Into<usize>) -> Option<u8> {
        let addr: usize = addr.into();
        self.banks.Vram.get(addr)
    }

    pub fn get_mut(&mut self, addr: impl Into<usize>) -> Result<&mut u8> {
        let addr: usize = addr.into();
        if addr <= ROM_END {
            bail!("Cannot mutate ROM {addr:#06x?}");
        } else {
            if addr > BANK_END {
                return Ok(&mut self.builtin_ram[mask(addr)]);
            }

            use BankId as B;
            match self.selected {
                B::Builtin => Ok(&mut self.builtin_ram[mask(addr)]),
                oth => Ok(self
                    .banks
                    .get_mut(oth.clone())
                    .map(|b| b.unwrap())?
                    .get_mut(addr)
                    .unwrap()),
            }
        }
    }

    pub fn set(&mut self, addr: impl Into<usize>, val: u8) -> Result<()> {
        let addr: usize = addr.into();
        let b = self.get_mut(addr)?;
        *b = val;
        Ok(())
    }
}