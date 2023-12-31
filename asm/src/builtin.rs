#![doc = include_str!("builtin/core/README.md")]

use macros::modules;

modules! {
    pub static "builtin/" BUILTIN = {
        core::{
            sys,
            macros::{
                util,
                call,
                clear,
                jmp,
                logic,
                send,
                math::{add, sub}
            }
        },
        std::{
            sleep,
            math::{
                mul::{mul, mul16},
                shift::{lsh, lsh16, rsh}
            },
            gfx::{
                frame,
                grid::{
                    point,
                    cfg,
                    block,
                }
            }
        }
    }
}
