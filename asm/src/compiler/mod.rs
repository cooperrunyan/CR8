pub mod builtin;
pub mod directive;
pub mod eval;
pub mod parse;

use directive::*;

use regex::Regex;

use self::{
    eval::{input::InputInstruction, Line},
    parse::expr,
};

pub fn compile(input: String) -> Compiled {
    let file = remove_comments(&input);
    let file = builtin::with_builtins(&file);
    let file = import::include(&file);
    let (file, defs) = def::collect(&file);
    let (file, mut labels) = store::collect(&file);
    let (file, macros) = mac::source(&file);
    let file = expr::parse(&file, &defs);

    let mut instructions: Vec<InputInstruction> = vec![];

    for line in file.split('\n') {
        match eval::line(line, instructions.len()) {
            None => continue,
            Some(x) => match x {
                Line::InputInstruction(i) => instructions.push(i),
                Line::Label((name, index)) => {
                    if labels.contains_key(&name) {
                        panic!("Attempting to set label: {name} twice");
                    }
                    labels.insert(name, index);
                }
            },
        }
    }

    let mut bin: Vec<u8> = vec![];

    for instruction in instructions {
        bin.append(&mut eval::instruction(instruction, &defs, &macros, &labels))
    }

    Compiled {
        bin,
        labels: labels.keys().map(|i| i.to_owned()).collect::<Vec<_>>(),
        macros: macros.keys().map(|i| i.to_owned()).collect::<Vec<_>>(),
    }
}

#[derive(Debug)]
pub struct Compiled {
    pub bin: Vec<u8>,
    pub labels: Vec<String>,
    pub macros: Vec<String>,
}

fn remove_comments(file: &str) -> String {
    let comm_re = Regex::new(r"(?sm);.*?$").unwrap();
    comm_re.replace_all(&file, "").to_string()
}
