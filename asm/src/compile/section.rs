use std::collections::HashMap;

use cfg::{op::Operation, reg::Register};

use super::{expr, num::try_read_num, scan::SymbolType, Arg};

pub fn compile_section(
    section: &str,
    section_start: &usize,
    section_index_map: &HashMap<String, usize>,
    variables: &HashMap<String, usize>,
    symbols: &HashMap<String, SymbolType>,
) -> Vec<u8> {
    use Arg::{Byte, Register as Reg};
    use Operation::*;

    let mut instructions = vec![];

    for line in section.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let (inst, args) = line.split_once(' ').unwrap_or((line, ""));
        let args = args
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|arg| {
                if arg.starts_with('%') {
                    Reg(Register::from(arg))
                } else {
                    match try_read_num(arg) {
                        Ok(num) => Byte(num as u8),
                        Err(_) => {
                            let parsed = expr::parse(
                                arg,
                                instructions.len() + *section_start,
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
                                Err(e) => match section_index_map.get(e.to_string().as_str()) {
                                    None => panic!(
                                        "Expression failed to resolve: '{arg}', to '{parsed}'. {e}"
                                    ),
                                    Some(v) => v.to_owned() as u128,
                                },
                            };
                            let b = (val >> (byte_num * 8)) as u8;
                            Byte(b)
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        let operation = Operation::from(inst);

        let op_byte = (operation as u8) << 1;

        match (operation, args.as_slice()) {
            (LW | SW | PUSH | POP | JNZ, &[Reg(r)]) => {
                instructions.push(op_byte);
                instructions.push(r as u8);
            }
            (LW, &[Reg(r), Byte(b0), Byte(b1)]) => {
                instructions.push(op_byte | 1);
                instructions.push(r as u8);
                instructions.push(b0);
                instructions.push(b1);
            }
            (SW, &[Byte(b0), Byte(b1), Reg(r)]) => {
                instructions.push(op_byte | 1);
                instructions.push(b0);
                instructions.push(b1);
                instructions.push(r as u8);
            }
            (PUSH | JNZ, &[Byte(b)]) => {
                instructions.push(op_byte | 1);
                instructions.push(b);
            }
            (OUT, &[Byte(b), Reg(r)]) => {
                instructions.push(op_byte | 1);
                instructions.push(b);
                instructions.push(r as u8);
            }
            (MOV | IN | OUT | CMP | ADC | SBB | OR | NOR | AND, &[Reg(r0), Reg(r1)]) => {
                instructions.push(op_byte);
                instructions.push(r0 as u8);
                instructions.push(r1 as u8);
            }
            (MOV | IN | CMP | ADC | SBB | OR | NOR | AND, &[Reg(r), Byte(b)]) => {
                instructions.push(op_byte | 1);
                instructions.push(r as u8);
                instructions.push(b);
            }
            _ => {
                panic!(
                    "Unexpected instruction/arg combination: '{inst}', {:?}",
                    args
                )
            }
        };
    }

    instructions
}
