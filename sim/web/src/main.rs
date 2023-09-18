use gloo_events::EventListener;
use gloo_timers::callback::Interval;
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

    let (int, kd, ku) = run(include_bytes!("../../../target/test.bin"))?;

    int.forget();
    kd.forget();
    ku.forget();

    Ok(())
}

fn run(bin: &[u8]) -> Result<(Interval, EventListener, EventListener), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("canvas")?;

    canvas.set_attribute("id", "cr8")?;
    canvas.set_attribute("height", "1024")?;
    canvas.set_attribute("width", "1024")?;

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
    context.set_fill_style(&JsValue::from_str("#ffffff"));

    let state = Arc::new(State {
        mem: RwLock::new(Mem::new(bin)),
        dev: RwLock::new(Devices::default()),
        cr8: RwLock::new(CR8::new()),
    });

    let int = {
        let mut tick: usize = 0;
        let state = state.clone();
        Interval::new(1, move || {
            let status = { state.dev.read().unwrap().sysctrl.state };

            if status == 1 {
                return;
            }

            let context = context.clone();
            for _ in 0..4000 {
                state
                    .cr8
                    .write()
                    .unwrap()
                    .cycle(&state.mem, &state.dev)
                    .unwrap();

                let byte = state.mem.read().unwrap().get_vram(tick).unwrap();

                for j in 0..8 {
                    let j = 7 - j;
                    let v = (byte >> j) & 1;

                    let by = tick >> 5;
                    let bx = tick & 0b11111;
                    let x = (bx as f64) * 8.0 + (7 - j) as f64;
                    let y = by as f64;

                    if v == 1 {
                        context.fill_rect(x * 4.0, y * 4.0, 4.0, 4.0);
                    } else {
                        context.clear_rect(x * 4.0, y * 4.0, 4.0, 4.0);
                    }
                }

                let newtick = if tick >= 0x3fff { 0 } else { tick + 1 };

                tick = newtick;
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
                    return;
                }
                "2" => {
                    state.cr8.read().unwrap().debug(
                        &state.mem.read().unwrap(),
                        state.dev.read().unwrap().snapshot(),
                    );
                    return;
                }
                _ => return,
            };
            // state.dev.write().unwrap().keyboard.set(k, false);
        })
    };

    Ok((int, keydown, keyup))
}
