use std::fs::OpenOptions;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output,
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

impl Config {
    pub fn from_argv() -> Self {
        let mut input: Option<Input> = None;
        let mut output = Output::None;

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
                _ => {}
            }
        }
        if input.is_none() {
            panic!("Did not specify input file")
        }
        let input = input.unwrap();

        Self { input, output }
    }
}
