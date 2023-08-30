use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub input: PathBuf,
    pub output: PathBuf,
    pub literal: String,
}

impl Config {
    pub fn from_argv() -> Self {
        let mut input = String::new();
        let mut output = String::new();

        for (i, arg) in std::env::args().enumerate() {
            if arg == "-i" {
                if !input.is_empty() {
                    panic!("Attempted to set -i flag twice");
                }
                input = std::env::args().nth(i + 1).unwrap_or_default();
            }
            if arg == "-o" {
                if !output.is_empty() {
                    panic!("Attempted to set -o flag twice");
                }
                output = std::env::args().nth(i + 1).unwrap_or_default();
            }
        }
        if input.is_empty() {
            panic!("Did not specify input file")
        }
        if output.is_empty() {
            panic!("Did not specify output file")
        }

        Self {
            input: input.into(),
            literal: "".into(),
            output: output.into(),
        }
    }
}
