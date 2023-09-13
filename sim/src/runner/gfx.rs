use anyhow::{anyhow, Result};
use asm::op::Operation;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{cr8::Joinable, devices::DeviceID};

use super::Runner;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

impl Runner {
    pub fn run(self) -> Result<Self> {
        println!("Starting");
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
            WindowBuilder::new()
                .with_title("CR8")
                .with_inner_size(scaled_size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };

        let runner = Arc::new(Mutex::new(self));

        let mut ticker = 0x8000_i64;

        event_loop.run(move |event, _, control_flow| {
            ticker += 1;
            let start = Instant::now();

            if ticker > 0xC000 {
                ticker = 0x8000;
            }

            if let Event::RedrawRequested(_) = event {
                let runner = runner.clone();
                let (byte, ticks) = {
                    let mut runner = runner.lock().unwrap();
                    runner.cycle(ticker as u16).unwrap()
                };

                let i = ticker - 0x8000;

                let frame = pixels.frame_mut();

                if (i + 1) * 32 > 0xffff {
                    return window.request_redraw();
                }

                for j in 0..8 {
                    let v = if byte >> (7 - j) & 1 == 1 { 0xff } else { 0 };
                    frame[((i * 32) + j * 4) as usize] = v;
                    frame[(((i * 32) + j * 4) + 1) as usize] = v;
                    frame[(((i * 32) + j * 4) + 2) as usize] = v;
                    frame[(((i * 32) + j * 4) + 3) as usize] = 255;
                }

                if ticker == 0x8000 {
                    if let Err(err) = pixels.render() {
                        println!("ERROR: {err:#?}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                let target = Duration::from_nanos(250) * ticks.into();
                let elapsed = Instant::now().duration_since(start);

                if elapsed < target {
                    thread::sleep(target - elapsed);
                }

                window.request_redraw();
            }
        });
    }

    pub fn cycle(&mut self, target: u16) -> Result<(u8, u8)> {
        let mut cr8 = self.cr8.lock().map_err(|_| anyhow!("Mutex poisoned"))?;

        if let Some(dev) = self.devices.get(DeviceID::SysCtrl) {
            let status = {
                dev.lock()
                    .map_err(|_| anyhow!("Failed to lock mutex"))?
                    .send()?
            };

            if status >> 1 & 1 == 1 {
                cr8.debug();
            }

            if status == 0x01 {
                return Ok((0, 0));
            }
        }

        let pc = cr8.pc().join();
        let inst = cr8.memory.get(0, pc);

        let op = Runner::oper(pc, inst >> 4)?;
        let is_imm = (inst & 0b00001000) == 0b00001000;
        let reg_bits = inst & 0b00000111;

        let b0: u8 = cr8.memory.get(0, pc + 1);
        let b1: u8 = cr8.memory.get(0, pc + 2);

        use Operation as O;

        let ticks = match (op, is_imm) {
            (O::LW, true) => cr8.lw_imm16(Runner::reg(pc, reg_bits)?, (b0, b1)),
            (O::LW, false) => cr8.lw_hl(Runner::reg(pc, reg_bits)?),
            (O::SW, true) => cr8.sw_imm16((b0, b1), Runner::reg(pc, reg_bits)?),
            (O::SW, false) => cr8.sw_hl(Runner::reg(pc, reg_bits)?),
            (O::MOV, true) => cr8.mov_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::MOV, false) => cr8.mov_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::PUSH, true) => cr8.push_imm8(b0),
            (O::PUSH, false) => cr8.push_reg(Runner::reg(pc, reg_bits)?),
            (O::POP, _) => cr8.pop(Runner::reg(pc, reg_bits)?),
            (O::MB, _) => cr8.set_mb(b0),
            (O::JNZ, true) => cr8.jnz_imm8(b0),
            (O::JNZ, false) => cr8.jnz_reg(Runner::reg(pc, reg_bits)?),
            (O::IN, true) => cr8.in_imm8(&self.devices, Runner::reg(pc, reg_bits)?, b0),
            (O::IN, false) => cr8.in_reg(
                &self.devices,
                Runner::reg(pc, reg_bits)?,
                Runner::reg(pc, b0)?,
            ),
            (O::OUT, true) => cr8.out_imm8(&self.devices, b0, Runner::reg(pc, reg_bits)?),
            (O::OUT, false) => cr8.out_reg(
                &self.devices,
                Runner::reg(pc, reg_bits)?,
                Runner::reg(pc, b0)?,
            ),
            (O::CMP, true) => cr8.cmp_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::CMP, false) => cr8.cmp_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::ADC, true) => cr8.adc_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::ADC, false) => cr8.adc_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::SBB, true) => cr8.sbb_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::SBB, false) => cr8.sbb_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::OR, true) => cr8.or_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::OR, false) => cr8.or_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::NOR, true) => cr8.nor_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::NOR, false) => cr8.nor_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
            (O::AND, true) => cr8.and_imm8(Runner::reg(pc, reg_bits)?, b0),
            (O::AND, false) => cr8.and_reg(Runner::reg(pc, reg_bits)?, Runner::reg(pc, b0)?),
        };

        let ticks = ticks?;

        for _ in 0..ticks {
            cr8.inc_pc();
        }

        Ok((cr8.memory.get(1, target), ticks))
    }
}
