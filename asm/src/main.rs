use std::fs::OpenOptions;
use std::io::Write;

use asm::{compile, Config};

fn main() {
    let cfg = Config::from_argv();
    let output = cfg.output.clone();
    let binary = compile(cfg);

    dbg!(&binary);

    let mut options = OpenOptions::new();
    let mut file = options
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(output)
        .expect("Failed to open output file");

    let _ = file.write_all(&binary);
}
