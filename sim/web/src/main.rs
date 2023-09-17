use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

use gloo_timers::callback::Interval;
use sim::cr8::mem::Mem;
use sim::cr8::CR8;
use sim::devices::Devices;

use std::f64;

struct State {
    mem: RwLock<Mem>,
    cr8: RwLock<CR8>,
    dev: RwLock<Devices>,
}

fn main() -> Result<(), JsValue> {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.create_element("canvas")?;

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

    let state = State {
        mem: RwLock::new(Mem::new(include_bytes!("../../../target/test.bin"))),
        dev: RwLock::new(Devices::default()),
        cr8: RwLock::new(CR8::new()),
    };

    let mut tick: usize = 0;

    let int = Interval::new(1, move || {
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
    });

    int.forget();

    Ok(())
}
