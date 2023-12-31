use gloo_events::EventListener;
use gloo_timers::callback::Interval;
use log::info;
use sim::cr8::mem::Mem;
use sim::cr8::CR8;
use sim::devices::keyboard::Key;
use sim::devices::Devices;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use std::f64;

struct State {
    mem: RwLock<Mem>,
    cr8: RwLock<CR8>,
    dev: RwLock<Devices>,
}

fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    let (int, kd, ku) = run(include_bytes!("../../../target/web.bin"))?;

    int.forget();
    kd.forget();
    ku.forget();

    Ok(())
}

const SCALE: f64 = 8.0;

const HZ: usize = 4_000_000;
const HZ_10MS: usize = HZ / 1000;

fn run(bin: &[u8]) -> Result<(Interval, EventListener, EventListener), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("canvas")?;

    canvas.set_attribute("id", "cr8")?;
    canvas.set_attribute("height", (128.0 * SCALE).to_string().as_str())?;
    canvas.set_attribute("width", (128.0 * SCALE).to_string().as_str())?;

    body.append_child(&canvas)?;

    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = Rc::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap(),
    );
    // context.set_fill_style(&JsValue::from_str("#ffffff"));

    let state = Arc::new(State {
        mem: RwLock::new(Mem::new(bin)),
        dev: RwLock::new(Devices::default()),
        cr8: RwLock::new(CR8::new()),
    });

    let int = {
        let mut i: usize = 0;
        let state = state.clone();

        Interval::new(1, move || {
            let status = { state.dev.read().unwrap().sysctrl.state };

            if status == 1 {
                info!("Halt");
                return;
            }

            let context = context.clone();
            let mut ticks = 0;

            loop {
                if ticks >= HZ_10MS {
                    break;
                }

                let ticks_in_this_cycle = {
                    let mut cr8 = state.cr8.write().unwrap();
                    cr8.cycle(&state.mem, &state.dev).unwrap() as usize
                };

                // When it does a 'jmp' it returns 0 ticks, but we still want the renderer to
                // progress
                let ticks_in_this_cycle = ticks_in_this_cycle.max(1);

                ticks += ticks_in_this_cycle;

                let mem = state.mem.read().unwrap();

                for _ in 0..ticks_in_this_cycle {
                    let byte = mem.get_vram(i).unwrap();
                    let r = ((byte >> 4) & 0b11) * 64;
                    let g = ((byte >> 2) & 0b11) * 64;
                    let b = (byte & 0b11) * 64;
                    let color = format!("#{r:02x}{g:02x}{b:02x}ff");

                    context.set_fill_style(&JsValue::from_str(&color));
                    let x = i & 0b1111111;
                    let y = i >> 7;
                    context.fill_rect(x as f64 * SCALE, y as f64 * SCALE, SCALE, SCALE);

                    //                     for j in 0..8 {
                    //                         let j = 7 - j;
                    //                         let v = (byte >> j) & 1;
                    //
                    //                         let by = i >> 5;
                    //                         let bx = i & 0b11111;
                    //                         let x = (bx as f64) * 8.0 + (7 - j) as f64;
                    //                         let y = by as f64;
                    //
                    //                         if v == 1 {
                    //                             context.fill_rect(x * SCALE, y * SCALE, SCALE, SCALE);
                    //                         } else {
                    //                             context.clear_rect(x * SCALE, y * SCALE, SCALE, SCALE);
                    //                         }
                    //                     }

                    i = (i + 1) & 0x3fff;
                }
            }
        })
    };

    let keydown = {
        let state = state.clone();
        EventListener::new(&window, "keydown", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            let k = match event.key().as_str() {
                "ArrowLeft" | "a" => Key::Left,
                "ArrowRight" | "d" => Key::Right,
                "ArrowDown" | "s" => Key::Down,
                "ArrowUp" | "w" => Key::Up,
                " " => Key::Space,
                "r" => Key::R,
                "=" => Key::Plus,
                "-" => Key::Minus,
                _ => return,
            };
            state.dev.write().unwrap().keyboard.set(k, true);
        })
    };

    let keyup = {
        let state = state.clone();
        EventListener::new(&window, "keyup", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            match event.key().as_str() {
                // Reset state
                // "ArrowLeft" | "a" => Key::Left,
                // "ArrowRight" | "d" => Key::Right,
                // "ArrowDown" | "s" => Key::Down,
                // "ArrowUp" | "w" => Key::Up,
                // " " => Key::Space,
                // "r" => Key::R,
                // "=" => Key::Plus,
                // "-" => Key::Minus,
                "1" => {
                    let mut d = state.dev.write().unwrap();
                    if d.sysctrl.state & 1 == 1 {
                        d.sysctrl.state = 0
                    } else {
                        d.sysctrl.state |= 1
                    }
                }
                "2" => {
                    state.cr8.read().unwrap().debug(
                        &state.mem.read().unwrap(),
                        state.dev.read().unwrap().snapshot(),
                    );
                }
                _ => (),
            }
            // state.dev.write().unwrap().keyboard.set(k, false);
        })
    };

    Ok((int, keydown, keyup))
}
