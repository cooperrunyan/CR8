use asm::compiler::{Compiler, Config};

fn main() {
    let config = Config::from_argv();
    let mut compiler = Compiler::new(&config);

    compiler.push(config.input);

    let _ = config.output.write(&compiler.compile());
}
