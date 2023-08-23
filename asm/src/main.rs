use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;

mod args;

fn main() {
    let args = args::collect();
    let binary = asm::compile(&args.source_file);

    let mut options = OpenOptions::new();
    let mut file = options
        .write(true)
        .append(false)
        .open(args.output)
        .expect("Failed to open output file");

    let _ = file.write_all(&binary);
}
