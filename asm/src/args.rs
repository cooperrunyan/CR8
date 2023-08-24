use std::fs;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub source_file: String,
    pub output: String,
}

pub fn collect() -> Args {
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

    let source_file = fs::read(&input)
        .map(String::from_utf8)
        .expect("Could not read input file")
        .expect("Could not read input file");

    Args {
        input,
        output,
        source_file,
    }
}
