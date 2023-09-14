use anyhow::{anyhow, bail, Result};
use std::fmt::Debug;

fn mask(idx: u16) -> usize {
    (idx & 0x7fff) as usize
}

fn smallmask(idx: u16) -> usize {
    (idx & 0x3fff) as usize
}

pub struct Bank([u8; 0x4000]);

impl Default for Bank {
    fn default() -> Self {
        Self([0; 0x4000])
    }
}

impl Bank {
    pub fn get(&self, idx: u16) -> Option<u8> {
        self.0.get(smallmask(idx)).map(|b| *b)
    }

    pub fn get_mut(&mut self, idx: u16) -> Option<&mut u8> {
        self.0.get_mut(smallmask(idx))
    }

    pub fn set(&mut self, idx: u16, val: u8) -> Result<()> {
        let byte = self.0.get_mut(smallmask(idx)).unwrap();
        *byte = val;
        Ok(())
    }
}

define_banks! {
    pub enum BankId,
    pub struct BankCollection {
        Vram(0x01) if "gfx",
    }
}

#[derive(Debug)]
pub struct Mem {
    selected: BankId,
    rom: [u8; 0x8000],
    builtin_ram: [u8; 0x8000],
    pub(super) banks: BankCollection,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            selected: BankId::Builtin,
            rom: [0; 0x8000],
            builtin_ram: [0; 0x8000],
            banks: BankCollection::default(),
        }
    }
}

impl Mem {
    pub fn new(bin: &[u8]) -> Self {
        let mut rom = [0; 0x8000];
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

    pub fn get(&self, addr: u16) -> Result<u8> {
        if addr >> 15 == 0 {
            Ok(self.rom[mask(addr)])
        } else {
            if addr >> 14 != 0 {
                return Ok(self.builtin_ram[mask(addr)]);
            }

            use BankId as B;
            match self.selected {
                B::Builtin => Ok(self.builtin_ram[mask(addr)]),
                oth => match self.banks.get(oth.clone()).map(|b| b.unwrap())?.get(addr) {
                    Some(x) => Ok(x),
                    None => bail!(
                        "Address {addr:#?} as {:#?} not found in {oth:#?}.",
                        smallmask(addr)
                    ),
                },
            }
        }
    }

    pub fn get_mut(&mut self, addr: u16) -> Result<&mut u8> {
        if addr >> 15 == 0 {
            bail!("Cannot mutate ROM");
        } else {
            if addr >> 14 != 0 {
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
                    .unwrap_or(Err(anyhow!(
                        "Address {addr:#?} as {:#?} not found in {oth:#?}.",
                        smallmask(addr)
                    ))?)),
            }
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) -> Result<()> {
        let b = self.get_mut(addr)?;
        *b = val;
        Ok(())
    }
}
