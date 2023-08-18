use std::collections::HashMap;

use cfg::{
    mem::{Size, SizedValue},
    op::OpArg,
    reg::Register,
};

use crate::compiler::eval::input::InputArg;

#[derive(Debug)]
pub enum Arg {
    None,
    Register(Register),
    Immediate(SizedValue),
}

pub fn check_arg(
    arg: &Option<&InputArg>,
    op: &OpArg,
    labels: &HashMap<String, u16>,
) -> Option<Arg> {
    match op {
        OpArg::Immediate(sz) => match arg {
            Some(InputArg::Immediate(v)) => Some(Arg::Immediate(sz.val_u(v))),
            Some(InputArg::Index(i)) => match sz {
                Size::Word => Some(Arg::Immediate(SizedValue::Word(
                    labels.get(i).expect("Label not defined").to_owned(),
                ))),
                _ => None,
            },
            _ => None,
        },
        OpArg::None => match arg {
            None => Some(Arg::None),
            _ => None,
        },
        OpArg::Register => match arg {
            Some(InputArg::Register(r)) => Some(Arg::Register(r.to_owned())),
            _ => None,
        },
    }
}
