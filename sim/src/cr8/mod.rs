use std::collections::HashMap;
use std::time::Duration;

use asm::reg::Register;
use typed_builder::TypedBuilder;

use crate::device::Device;

use self::mem::Mem;

pub mod mem;

mod debug;
mod exec;
mod inst;
mod probe;

pub const STACK: u16 = 0xFC00;
pub const STACK_END: u16 = 0xFEFF;

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

        cr8.connect_devices();

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
