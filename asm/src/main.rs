#[macro_use]
extern crate lazy_static;

use cfg::reg::Register;
use compile::{
    macros::{self, Macro, MacroArg},
    Definition, LiteralNumber, Size,
};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::compile::parse_num;

mod args;
mod compile;

fn main() {
    let (input, input_path, output_path) = match args::parse() {
        Ok(r) => r,
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    let mut instructions: Vec<Item> = vec![];

    let mut files_sourced = vec![];
    let file =
        compile::include_import_statements(&input, PathBuf::from(input_path), &mut files_sourced);

    let comm_re = Regex::new(r"(?sm);.*?$").unwrap();
    let space_re = Regex::new(r"( )+").unwrap();
    let label_re = Regex::new(r"^(\w+):$").unwrap();

    let file: String = comm_re.replace_all(&file, "").into();

    let (file, refs) = compile::read_def_stmts(&file);
    let (file, stores) = compile::get_store_stmts(&file);
    let (file, macros) = compile::macros::source(&file);

    let mut labels = vec![];

    for line in file.split('\n') {
        let line = line.trim();
        if label_re.is_match(line) {
            let label_name = line.trim_end_matches(':').to_string();
            if labels.contains(&label_name) {
                panic!("Label: {label_name} already defined");
            }
            labels.push(label_name.clone());
            instructions.push(Item::Label(label_name));
            continue;
        }
        if line.is_empty() {
            continue;
        }

        let instruction = Instruction::from(line);
        instructions.push(Item::Instruction(instruction));
    }

    println!("{}", file);
}

fn evaluate_instruction(
    inst: Instruction,
    defs: &HashMap<String, Definition>,
    macros: &HashMap<String, Vec<Macro>>,
    labels: &[String],
    stores: &HashMap<String, usize>,
) -> Vec<u8> {
    let mut instructions = vec![];

    let mut macroed = false;
    if macros.contains_key(&inst.inst) {
        for mac in macros.get(&inst.inst).unwrap_or(&Vec::new()) {
            let mut possible = true;
            for (i, mac_arg) in mac.args.iter().enumerate() {
                if let Some(ar) = inst.args.get(i) {
                    match (mac_arg, ar) {
                        (&MacroArg::Immediate(_), &Arg::Immediate(_)) => {}
                        (&MacroArg::Register(_), &Arg::Register(_)) => {}
                        _ => {
                            possible = false;
                            break;
                        }
                    }
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

                let instruction = Instruction::from(line);
                instructions.append(&mut evaluate_instruction(
                    instruction,
                    defs,
                    macros,
                    labels,
                    stores,
                ));
            }
        }
    }

    // Do the same thing as the macro one, but this time check against native operations

    instructions
}

enum Item {
    Instruction(Instruction),
    Label(String),
}

struct Instruction {
    pub inst: String,
    pub args: Vec<Arg>,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let value = value.trim();

        let (ident, args) = value.split_at(value.find(|c| c == ' ').unwrap());
        let args = args
            .split(',')
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(|a| {
                if let Some(reg) = a.strip_prefix('%') {
                    Arg::Register(Register::from(reg))
                } else if a.starts_with('$') {
                    Arg::Immediate(parse_num(a))
                } else if a.starts_with('[') && a.ends_with(']') {
                    let a = a.trim_start_matches('[').trim_end_matches(']');
                    Arg::Index(a.to_string())
                } else {
                    panic!("Bad argument")
                }
            })
            .collect::<Vec<_>>();

        Self {
            inst: ident.to_string(),
            args,
        }
    }
}

enum Arg {
    Register(Register),
    Immediate(u64),
    Index(String),
}

impl ToString for Arg {
    fn to_string(&self) -> String {
        match self {
            Arg::Immediate(n) => format!("${n}D",),
            Arg::Index(r) => format!("[{}]", r.to_string()),
            Arg::Register(r) => format!("%{}", r.to_string()),
        }
    }
}
