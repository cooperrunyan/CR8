#[macro_use]
extern crate lazy_static;

mod compile;

use std::collections::HashMap;

use cfg::{
    op::{Arg as NativeArg, Operation, NATIVE},
    reg::Register,
};
use compile::{
    expr,
    num::try_read_num,
    scan::{scan, SymbolType},
};

const PADDING: u64 = 0;

pub fn compile(source: &str) -> Vec<u8> {
    let (mut ctx, _) = scan(source);

    ctx.resolve_macros();

    if !ctx.sections.contains_key("main") {
        panic!("Expected a 'main' label, none found");
    }

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut acc = 0;

    for (sym, ty) in ctx.symbols.iter() {
        match ty {
            SymbolType::MemByte => {
                variables.insert(sym.to_string(), acc);
                acc += 1;
            }
            SymbolType::MemWord => {
                variables.insert(sym.to_string(), acc);
                acc += 2;
            }
            SymbolType::MemDouble => {
                variables.insert(sym.to_string(), acc);
                acc += 4;
            }
            _ => {}
        }
    }

    let mut sections = ctx.sections;
    let mut section_sizes: HashMap<String, usize> = HashMap::new();

    for (name, section) in sections.iter() {
        let mut size: usize = 0;
        for line in section.lines() {
            let (inst, args) = line.trim().split_once(' ').unwrap_or((line.trim(), ""));
            // JNZ is always 1 byte
            if inst == "jnz" {
                size += 1;
                continue;
            }

            if inst.is_empty() {
                continue;
            }

            size += 1;

            for (i, arg) in args
                .split(',')
                .map(|a| a.trim())
                .filter(|a| !a.is_empty())
                .enumerate()
            {
                if !(i == 0 && arg.starts_with('%')) {
                    size += 1;
                }
            }
        }
        section_sizes.insert(name.to_string(), size);
    }

    let main_size = section_sizes.remove("main").unwrap();
    let main_section = sections.remove("main").unwrap();

    let mut section_index_map: HashMap<String, usize> = HashMap::new();

    section_index_map.insert("main".to_string(), 0);

    let mut acc = main_size;

    for (name, size) in section_sizes.iter() {
        acc += PADDING as usize;
        section_index_map.insert(name.to_string(), acc.to_owned());
        acc += size
    }

    let mut bin: Vec<u8> = vec![];

    bin.append(&mut compile_section(
        &main_section,
        &section_index_map.get("main").unwrap(),
        &section_index_map,
        &variables,
        &ctx.symbols,
    ));

    for (name, section) in sections.iter() {
        let mut sect = compile_section(
            section,
            &section_index_map.get(name).unwrap(),
            &section_index_map,
            &variables,
            &ctx.symbols,
        );

        bin.append(&mut sect);
    }

    bin
}

fn compile_section(
    section: &str,
    section_start: &usize,
    section_index_map: &HashMap<String, usize>,
    variables: &HashMap<String, usize>,
    symbols: &HashMap<String, SymbolType>,
) -> Vec<u8> {
    let mut instructions = vec![];

    for line in section.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let (inst, args) = line.split_once(' ').unwrap_or((line, ""));
        let args = args
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|arg| {
                if arg.starts_with("%") {
                    Arg::Register(Register::from(arg))
                } else {
                    match try_read_num(arg) {
                        Ok(num) => Arg::Byte(num as u8),
                        Err(_) => {
                            let parsed = expr::parse(
                                arg,
                                instructions.len() + section_start.clone(),
                                symbols,
                                section_index_map,
                                variables,
                            );
                            let parsed = parsed.replace(['[', ']'], "");
                            let (val, byte_num) =
                                parsed.split_once("::").unwrap_or((parsed.as_str(), "0"));
                            let byte_num: u8 =
                                byte_num.parse().expect("Failed to parse intended byte");
                            let val: u128 = match val.parse() {
                                Ok(v) => v,
                                Err(e) => {
                                    panic!(
                                        "Expression failed to resolve: '{arg}', to '{parsed}'. {e}"
                                    )
                                }
                            };
                            let b = (val >> (byte_num * 8)) as u8;
                            Arg::Byte(b)
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let operation = Operation::from(inst);
        let instruction_matches = match NATIVE.get(&operation) {
            None => panic!("Unknown instruction: {}", inst),
            Some(a) => a,
        };

        let mut found_match = false;

        for matched_args in instruction_matches {
            if matched_args.len() != args.len() {
                continue;
            }
            let mut possible = true;
            for (i, matched_arg) in matched_args.iter().enumerate() {
                let arg = args.get(i).unwrap();

                match (matched_arg, arg) {
                    (NativeArg::Byte, Arg::Byte(_)) => {}
                    (NativeArg::Register, Arg::Register(_)) => {}
                    _ => possible = false,
                }
            }
            if !possible {
                continue;
            }
            found_match = true;
            break;
        }

        if !found_match {
            panic!("Invalid argument types for '{inst}': {:?}", args)
        }

        let opcode = (operation as u8) << 4;

        let mut is_imm = 0;
        for arg in args.iter() {
            match arg {
                Arg::Byte(_) => is_imm = 0b00001000,
                _ => {}
            }
        }

        if args.is_empty() {
            instructions.push(opcode | is_imm);
            continue;
        }

        let mut args = args;
        let first = args.remove(0);

        if inst == "jnz" {
            match first {
                Arg::Byte(0) => instructions.push(opcode | 0b00001000 | 0),
                Arg::Byte(_) => instructions.push(opcode | 0b00001000 | 1),
                Arg::Register(r) => instructions.push(opcode | r as u8),
            };
            continue;
        }

        match &first {
            Arg::Register(r) => instructions.push(opcode | is_imm | (r.clone() as u8)),
            Arg::Byte(b) => {
                instructions.push(opcode | is_imm);
                instructions.push(b.to_owned())
            }
        };

        for arg in args {
            let v = match arg {
                Arg::Byte(b) => b,
                Arg::Register(r) => r as u8,
            };
            instructions.push(v)
        }
    }

    instructions
}

#[derive(Debug)]
enum Arg {
    Byte(u8),
    Register(Register),
}
