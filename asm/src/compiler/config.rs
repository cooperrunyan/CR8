use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output,
    pub(super) debug: DebugInfo,
}

#[derive(Debug, Clone)]
pub enum Input {
    Raw(String),
    File(String),
}

#[derive(Debug, Clone)]
pub enum Output {
    File(String),
    None,
}

impl Output {
    pub fn write(&self, bin: &Vec<u8>) -> Result<(), io::Error> {
        match self {
            Output::None => Ok(()),
            Output::File(f) => {
                let mut options = OpenOptions::new();
                let mut file = options
                    .write(true)
                    .truncate(true)
                    .append(false)
                    .create(true)
                    .open(f)?;

                file.write_all(bin)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct DebugInfo {
    pub(super) files: bool,
    pub(super) labels: bool,
    pub(super) macros: bool,
    pub(super) expr: bool,
    pub(super) bin: bool,
}

impl Config {
    pub fn from_argv() -> Self {
        let mut input: Option<Input> = None;
        let mut output = Output::None;
        let mut debug = DebugInfo::default();

        for (i, arg) in std::env::args().enumerate() {
            match arg.as_str() {
                "-i" | "--input" => {
                    if input.is_some() {
                        panic!("Attempted to set input flag twice");
                    }
                    input = Some(Input::File(std::env::args().nth(i + 1).unwrap_or_default()));
                }
                "-o" | "--output" => {
                    output = Output::File(std::env::args().nth(i + 1).unwrap_or_default());
                }
                "-d" | "--debug" => match std::env::args().nth(i + 1) {
                    None => {}
                    Some(s) => match s.as_str() {
                        "files" => debug.files = true,
                        "macros" => debug.macros = true,
                        "bin" => debug.bin = true,
                        "labels" => debug.labels = true,
                        "expr" => debug.expr = true,
                        "all" => {
                            debug.files = true;
                            debug.macros = true;
                            debug.bin = true;
                            debug.expr = true;
                            debug.labels = true;
                        }
                        _ => {}
                    },
                },
                _ => {}
            }
        }
        if input.is_none() {
            panic!("Did not specify input file")
        }
        let input = input.unwrap();

        Self {
            input,
            output,
            debug,
        }
    }
}
