use std::collections::HashMap;

use cfg::{
    mem::{DEV_CONTROL, STACK},
    reg::Register,
};
use typed_builder::TypedBuilder;

use crate::device::{Control, Device};

mod debug;
mod exec;
mod inst;
mod probe;

fn join((l, h): (u8, u8)) -> u16 {
    ((h as u16) << 8) | (l as u16)
}

fn split(hl: u16) -> (u8, u8) {
    ((hl as u8), (hl >> 8) as u8)
}

#[derive(TypedBuilder)]
pub struct CR8Config {
    #[builder(default = 400)]
    pub tick_rate: u64,

    #[builder(default = Vec::new())]
    pub with_devices: Vec<(u8, Box<dyn Device>)>,

    #[builder(default = Vec::new())]
    pub mem: Vec<u8>,
}

pub struct CR8 {
    pub reg: [u8; 8],
    pub mem: [u8; 65536],
    pub dev: HashMap<u8, Box<dyn Device>>,
    pub tick_rate: u64,
}

impl CR8 {
    pub fn new(sim_cfg: CR8Config) -> Self {
        let mut cr8 = Self {
            reg: [0; 8],
            mem: [0; 65536],
            tick_rate: sim_cfg.tick_rate,
            dev: HashMap::new(),
        };

        cr8.set_sp(split(STACK));

        cr8.dev_add(DEV_CONTROL, Box::<Control>::default());

        for (port, dev) in sim_cfg.with_devices {
            cr8.dev_add(port, dev)
        }

        for (i, byte) in sim_cfg.mem.into_iter().enumerate() {
            cr8.mem[i] = byte
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
