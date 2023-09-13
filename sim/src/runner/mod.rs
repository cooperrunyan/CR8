use std::io::stdin;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};

use asm::{op::Operation, reg::Register};

use super::devices::{DeviceID, Devices};
use crate::cr8::{Joinable, Splittable, CR8, STACK};

mod config;

#[derive(Default)]
pub struct Runner {
    cr8: Arc<Mutex<CR8>>,
    devices: Devices,
    tickrate: Duration,
    step: bool,
}

impl Runner {
    pub fn new(tickrate: Duration, debug: bool, step: bool) -> Self {
        let mut cr8 = CR8::new();
        cr8.set_sp(STACK.split());
        cr8.debug = debug;

        Self {
            tickrate,
            step,
            devices: Devices::default(),
            cr8: Arc::new(Mutex::new(cr8)),
        }
    }

    pub fn debug(&self) -> Result<()> {
        self.cr8
            .lock()
            .map_err(|_| anyhow!("Failed to get a lock"))?
            .debug();
        Ok(())
    }

    pub fn load(&mut self, bin: &[u8]) -> Result<()> {
        {
            let mut c = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;
            c.memory.rom[..bin.len()].copy_from_slice(bin);
        }

        self.devices.connect(self.cr8.clone());

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        if self.step {
            for _ in stdin().lines() {
                let cnt = self.cycle()?;
                if !cnt {
                    return Ok(());
                }
            }
        } else {
            loop {
                let cnt = self.cycle()?;
                if !cnt {
                    break;
                }
            }
        }

        Ok(())
    }

    fn cycle(&mut self) -> Result<bool> {
        let mut cr8 = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;

        if let Some(dev) = self.devices.get(DeviceID::SysCtrl) {
            let status = dev
                .lock()
                .map_err(|_| anyhow!("Failed to lock mutex"))?
                .send()?;

            if status >> 1 & 1 == 1 {
                cr8.debug();
            }

            if status == 0x01 {
                return Ok(false);
            }
        }

        let inst = cr8.memory.get(cr8.mb, cr8.pc().join());

        let op = oper(inst >> 4)?;
        let is_imm = (inst & 0b00001000) == 0b00001000;
        let reg_bits = inst & 0b00000111;

        let b0: u8 = cr8.memory.get(cr8.mb, cr8.pc().join() + 1);
        let b1: u8 = cr8.memory.get(cr8.mb, cr8.pc().join() + 2);

        use Operation as O;

        let ticks = match (op, is_imm) {
            (O::LW, true) => cr8.lw_imm16(reg(reg_bits)?, (b0, b1)),
            (O::LW, false) => cr8.lw_hl(reg(reg_bits)?),
            (O::SW, true) => cr8.sw_imm16((b0, b1), reg(reg_bits)?),
            (O::SW, false) => cr8.sw_hl(reg(reg_bits)?),
            (O::MOV, true) => cr8.mov_imm8(reg(reg_bits)?, b0),
            (O::MOV, false) => cr8.mov_reg(reg(reg_bits)?, reg(b0)?),
            (O::PUSH, true) => cr8.push_imm8(b0),
            (O::PUSH, false) => cr8.push_reg(reg(reg_bits)?),
            (O::POP, _) => cr8.pop(reg(reg_bits)?),
            (O::MB, _) => cr8.set_mb(b0),
            (O::JNZ, true) => cr8.jnz_imm8(b0),
            (O::JNZ, false) => cr8.jnz_reg(reg(reg_bits)?),
            (O::IN, true) => cr8.in_imm8(&self.devices, reg(reg_bits)?, b0),
            (O::IN, false) => cr8.in_reg(&self.devices, reg(reg_bits)?, reg(b0)?),
            (O::OUT, true) => cr8.out_imm8(&self.devices, b0, reg(reg_bits)?),
            (O::OUT, false) => cr8.out_reg(&self.devices, reg(reg_bits)?, reg(b0)?),
            (O::CMP, true) => cr8.cmp_imm8(reg(reg_bits)?, b0),
            (O::CMP, false) => cr8.cmp_reg(reg(reg_bits)?, reg(b0)?),
            (O::ADC, true) => cr8.adc_imm8(reg(reg_bits)?, b0),
            (O::ADC, false) => cr8.adc_reg(reg(reg_bits)?, reg(b0)?),
            (O::SBB, true) => cr8.sbb_imm8(reg(reg_bits)?, b0),
            (O::SBB, false) => cr8.sbb_reg(reg(reg_bits)?, reg(b0)?),
            (O::OR, true) => cr8.or_imm8(reg(reg_bits)?, b0),
            (O::OR, false) => cr8.or_reg(reg(reg_bits)?, reg(b0)?),
            (O::NOR, true) => cr8.nor_imm8(reg(reg_bits)?, b0),
            (O::NOR, false) => cr8.nor_reg(reg(reg_bits)?, reg(b0)?),
            (O::AND, true) => cr8.and_imm8(reg(reg_bits)?, b0),
            (O::AND, false) => cr8.and_reg(reg(reg_bits)?, reg(b0)?),
        };

        for _ in 0..ticks? {
            cr8.inc_pc();
            self.devices.tick()?;
        }

        thread::sleep(self.tickrate);

        Ok(true)
    }
}

fn reg(byte: u8) -> Result<Register> {
    match Register::try_from(byte) {
        Ok(r) => Ok(r),
        Err(_) => bail!("Invalid register: {byte}"),
    }
}

fn oper(byte: u8) -> Result<Operation> {
    match Operation::try_from(byte) {
        Ok(r) => Ok(r),
        Err(_) => bail!("Invalid operation: {byte}"),
    }
}
