use std::collections::HashMap;
use std::time::Duration;

use asm::reg::Register;
use typed_builder::TypedBuilder;

use crate::device::{Control, Device};

mod debug;
mod exec;
mod inst;
mod probe;

pub const STACK: u16 = 0xFC00;
pub const STACK_END: u16 = 0xFEFF;
pub const DEV_CONTROL: u8 = 0x00;
pub const SIGNOP: u8 = 0x00;
pub const SIGHALT: u8 = 0x01;
pub const SIGPEEK: u8 = 0x02;
pub const SIGDBG: u8 = 0x03;

fn join((l, h): (u8, u8)) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

fn split(hl: u16) -> (u8, u8) {
    ((hl as u8), (hl >> 8) as u8)
}

#[derive(TypedBuilder)]
pub struct CR8Config {
    #[builder(default = Duration::from_millis(400))]
    pub tickrate: Duration,

    #[builder(default = Vec::new())]
    pub with_devices: Vec<(u8, Box<dyn Device>)>,

    #[builder(default = Vec::new())]
    pub mem: Vec<u8>,

    pub step: bool,
    pub debug: bool,
}

#[derive(Debug)]
pub struct Mem {
    rom: [u8; 0x8000],
    builtin_ram: [u8; 0x8000],
    banks: Vec<[u8; 0x4000]>,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            rom: [0; 0x8000],
            builtin_ram: [0; 0x8000],
            banks: vec![],
        }
    }
}

impl Mem {
    pub fn get(&self, bank: u8, addr: u16) -> u8 {
        if addr & 0b10000000_00000000 == 0 {
            return self.rom[(addr & 0b01111111_11111111) as usize];
        }
        if addr & 0b01000000_00000000 == 0 && bank != 0 {
            if self.banks.len() <= bank as usize - 1 {
                panic!("Attempted to address nonexistent bank: {}", bank - 1);
            }
            return self.banks[bank as usize - 1][(addr & 0b00111111_11111111) as usize];
        }
        return self.builtin_ram[(addr & 0b01111111_11111111) as usize];
    }

    pub fn set(&mut self, bank: u8, addr: u16, value: u8) {
        if addr & 0b10000000_00000000 == 0 {
            return;
        }
        if addr & 0b01000000_00000000 == 0 && bank != 0 {
            if self.banks.len() <= bank as usize - 1 {
                panic!("Attempted to address nonexistent bank: {}", bank - 1);
            }
            self.banks[bank as usize - 1][(addr & 0b00111111_11111111) as usize] = value;
        } else {
            self.builtin_ram[(addr & 0b01111111_11111111) as usize] = value;
        }
    }
}

pub struct CR8 {
    pub reg: [u8; 8],
    pub pcl: u8,
    pub pch: u8,
    pub spl: u8,
    pub sph: u8,
    pub mb: u8,
    pub memory: Mem,
    pub dev: HashMap<u8, Box<dyn Device>>,
    pub tickrate: Duration,
    pub debug: bool,
    pub step: bool,
}

impl CR8 {
    pub fn new(sim_cfg: CR8Config) -> Self {
        let mut cr8 = Self {
            reg: [0; 8],
            pcl: 0,
            pch: 0,
            spl: 0,
            sph: 0,
            mb: 0,
            memory: Mem::default(),
            tickrate: sim_cfg.tickrate,
            dev: HashMap::new(),
            step: sim_cfg.step,
            debug: sim_cfg.debug,
        };

        cr8.set_sp(split(STACK));

        cr8.memory.banks.push([0; 0x4000]); // VRAM

        cr8.dev_add(DEV_CONTROL, Box::<Control>::default());

        for (port, dev) in sim_cfg.with_devices {
            cr8.dev_add(port, dev)
        }

        for (i, byte) in sim_cfg.mem.into_iter().enumerate() {
            cr8.memory.rom[i] = byte
        }

        cr8
    }

    pub fn dev_add(&mut self, port: u8, dev: Box<dyn Device>) {
        if self.dev.contains_key(&port) {
            panic!("A device is already connected to port: {port}")
        }
        self.dev.insert(port, dev);
    }

    #[allow(dead_code)]
    pub fn dev_rm(&mut self, port: u8) {
        self.dev.remove(&port);
    }
}
