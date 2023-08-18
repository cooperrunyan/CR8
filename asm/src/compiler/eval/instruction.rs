use std::collections::HashMap;

use cfg::{
    mem::SizedValue,
    op::{Operation, NATIVE},
};

use crate::compiler::{
    directive::{
        def::Definition,
        mac::{Macro, MacroArg},
    },
    parse::input::{check_arg, Arg},
};

use super::input::{InputArg, InputInstruction};

pub fn instruction(
    inst: InputInstruction,
    defs: &HashMap<String, Definition>,
    macros: &HashMap<String, Vec<Macro>>,
    labels: &HashMap<String, u16>,
) -> Vec<u8> {
    let mut instructions = vec![];

    let mut macroed = false;
    if macros.contains_key(&inst.inst) {
        for mac in macros.get(&inst.inst).unwrap_or(&Vec::new()) {
            let mut possible = false;
            if mac.args.len() != inst.args.len() {
                continue;
            }
            for (i, mac_arg) in mac.args.iter().enumerate() {
                match mac_arg {
                    MacroArg::Immediate(_) => match inst.args.get(i) {
                        Some(InputArg::Immediate(_)) => {
                            possible = true;
                        }
                        _ => {}
                    },
                    MacroArg::Register(_) => match inst.args.get(i) {
                        Some(InputArg::Register(_)) => {
                            possible = true;
                        }
                        _ => {}
                    },
                    MacroArg::Index(_) => match inst.args.get(i) {
                        Some(InputArg::Index(_)) => {
                            possible = true;
                        }
                        _ => {}
                    },
                }
            }

            if !possible {
                continue;
            }

            macroed = true;

            let mut chunk = mac.body.to_string();
            for (i, mac_arg) in mac.args.iter().enumerate() {
                if let Some(ar) = inst.args.get(i) {
                    chunk = chunk.replace(&mac_arg.to_string(), &ar.to_string());
                }
            }

            for line in chunk.split('\n') {
                if line.is_empty() {
                    continue;
                }

                let inst = InputInstruction::from(line);
                instructions.append(&mut instruction(inst, defs, macros, labels));
            }
        }
    }

    if macroed {
        return instructions;
    }

    let arg_types = match NATIVE.get(&inst.inst) {
        None => panic!("Unknown instruction: {}", inst.inst),
        Some(a) => a,
    };

    let input_arg_1 = inst.args.get(0);
    let input_arg_2 = inst.args.get(1);

    let mut arg_1 = None;
    let mut arg_2 = None;

    for (a1, a2) in arg_types {
        let ra1 = match check_arg(&input_arg_1, &a1, labels) {
            None => continue,
            Some(v) => v,
        };

        let ra2 = match check_arg(&input_arg_2, &a2, labels) {
            None => continue,
            Some(v) => v,
        };

        arg_1 = Some(ra1);
        arg_2 = Some(ra2);
    }

    let arg_1 = match arg_1 {
        None => panic!("Invalid type arguments"),
        Some(a) => a,
    };

    let arg_2 = match arg_2 {
        None => panic!("Invalid type arguments"),
        Some(a) => a,
    };

    let op_code = Operation::from(inst.inst.as_str()) as u8;

    let is_imm = match (&arg_1, &arg_2) {
        (Arg::Immediate(_), _) => 1,
        (_, Arg::Immediate(_)) => 1,
        _ => 0,
    };

    let reg = match &arg_1 {
        Arg::Register(r) => r.clone() as u8,
        _ => 0,
    };

    let header = (op_code << 4) | (is_imm << 3) | reg;
    instructions.push(header);

    match &arg_1 {
        Arg::Immediate(imm) => match imm {
            SizedValue::Byte(b) => instructions.push(b.to_owned()),
            SizedValue::Word(w) => {
                instructions.push(w.to_owned() as u8);
                instructions.push((w.to_owned() >> 8) as u8);
            }
            SizedValue::Double(d) => {
                instructions.push(d.to_owned() as u8);
                instructions.push((d.to_owned() >> 8) as u8);
                instructions.push((d.to_owned() >> 16) as u8);
                instructions.push((d.to_owned() >> 24) as u8);
            }
        },
        _ => {}
    };
    match &arg_2 {
        Arg::Immediate(imm) => match imm {
            SizedValue::Byte(b) => instructions.push(b.to_owned()),
            SizedValue::Word(w) => {
                instructions.push(w.to_owned() as u8);
                instructions.push((w.to_owned() >> 8) as u8);
            }
            SizedValue::Double(d) => {
                instructions.push(d.to_owned() as u8);
                instructions.push((d.to_owned() >> 8) as u8);
                instructions.push((d.to_owned() >> 16) as u8);
                instructions.push((d.to_owned() >> 24) as u8);
            }
        },
        Arg::Register(r) => instructions.push(r.to_owned() as u8),
        _ => {}
    };

    instructions
}
