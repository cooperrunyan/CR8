use std::fs::write;

#[macro_use]
extern crate lazy_static;

mod args;
mod compiler;

fn main() {
    let (input, output_path) = match args::parse() {
        Ok(r) => r,
        Err(msg) => {
            println!("{msg}");
            return;
        }
    };

    let compiled = compiler::compile(input);

    write(output_path, compiled.bin).expect("Failed to write output");
}
