use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::cr8::mem::{BANK_LEN, RAM_START};

use super::Runner;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const SCALE: f64 = 4.0;

impl Runner {
    pub fn run(self) -> Result<Self> {
        println!("Starting");
        let event_loop = EventLoop::new();

        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * SCALE, HEIGHT as f64 * SCALE);
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

        let tickrate = Arc::new(self.tickrate);
        let runner = Arc::new(Mutex::new(self));

        let mut ticker = RAM_START;

        event_loop.run(move |event, _, control_flow| {
            let start = Instant::now();

            if let Event::RedrawRequested(_) = event {
                let runner = runner.clone();
                let ticks = {
                    let mut runner = runner.lock().unwrap();
                    runner.cycle().unwrap()
                };

                let byte = {
                    let runner = runner.lock().unwrap();
                    let cr8 = runner.cr8.lock().unwrap();
                    cr8.mem.get_vram(ticker as u16).unwrap_or(0)
                };

                let i = ticker - RAM_START;

                let frame = pixels.frame_mut();

                for j in 0..8 {
                    let v = if byte >> (7 - j) & 1 == 1 { 0xff } else { 0 };
                    frame.get_mut(((i * 32) + j * 4) as usize).map(|b| *b = v);
                    frame
                        .get_mut(((i * 32) + j * 4 + 1) as usize)
                        .map(|b| *b = v);
                    frame
                        .get_mut(((i * 32) + j * 4 + 2) as usize)
                        .map(|b| *b = v);
                    frame
                        .get_mut(((i * 32) + j * 4 + 3) as usize)
                        .map(|b| *b = 255);
                }

                if ticker == RAM_START {
                    if let Err(err) = pixels.render() {
                        println!("ERROR: {err:#?}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                let tr = tickrate.clone();

                let target = *tr * ticks as u32;
                let elapsed = Instant::now().duration_since(start);

                if elapsed < target {
                    thread::sleep(target - elapsed);
                }

                ticker += 1;

                if ticker >= RAM_START + BANK_LEN {
                    ticker = RAM_START;
                }

                window.request_redraw();
            }
        });
    }
}
