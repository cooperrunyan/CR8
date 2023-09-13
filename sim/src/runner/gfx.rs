use anyhow::Result;
use std::{
    sync::{Arc, Mutex},
    thread,
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::devices::DeviceID;

use super::Runner;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

impl Runner {
    pub fn run(self) -> Result<Self> {
        println!("Starting");
        let event_loop = EventLoop::new();
        // let mut input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let scaled_size = LogicalSize::new(WIDTH as f64 * 4.0, HEIGHT as f64 * 4.0);
            WindowBuilder::new()
                .with_title("Conway's Game of Life")
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

        let mut halted = false;

        event_loop.run(move |event, _, control_flow| {
            if halted {
                return;
            }
            // The one and only event that winit_input_helper doesn't have for us...
            if let Event::RedrawRequested(_) = event {
                let runner = runner.clone();
                let mut runner = runner.lock().unwrap();
                let contin = runner.cycle().unwrap();

                if !contin {
                    println!("HALTING");
                    halted = true;
                    return;
                }

                let communicator = runner.devices.get(DeviceID::Gfx).unwrap();
                let mut communicator = communicator.lock().unwrap();

                let byte = communicator.send().unwrap();
                let i = communicator.inspect();

                let frame = pixels.frame_mut();

                if (i + 1) * 32 > 0xffff {
                    return window.request_redraw();
                }
                //
                //                 if i * 32 < 0x3300 {
                //                     return window.request_redraw();
                //                 }

                //  for j in 0..7 {
                //     let v = if byte >> (7 - j) & 1 == 1 { 0xff } else { 0 };
                //     frame[(i * 32) + j * 4] = v;
                //     frame[((i * 32) + j * 4) + 1] = v;
                //     frame[((i * 32) + j * 4) + 2] = v;
                //     frame[((i * 32) + j * 4) + 3] = 255;
                // }

                for j in 0..8 {
                    let v = if byte >> (7 - j) & 1 == 1 { 0xff } else { 0 };
                    frame[(i * 32) + j * 4] = v;
                    frame[((i * 32) + j * 4) + 1] = v;
                    frame[((i * 32) + j * 4) + 2] = v;
                    frame[((i * 32) + j * 4) + 3] = 255;
                }

                if let Err(err) = pixels.render() {
                    println!("ERROR: {err:#?}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                thread::sleep(runner.tickrate);
                window.request_redraw();
            }
        });
    }
}
