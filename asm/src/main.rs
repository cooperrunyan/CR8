use std::env;
use std::sync::Arc;

use anyhow::Result;
use asm::compiler::{micro, Compiler, Config};

use env_logger::Env;

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .parse_env(Env::new().default_filter_or("asm=info"))
        .init();

    let config = Config::from_argv();

    if config.micro {
        let bin = micro::compile(config.input)?;
        let _ = config.output.write(&bin);

        return Ok(());
    }

    let mut compiler = Compiler::new();

    compiler.push(config.input, Arc::new(env::current_dir().unwrap()))?;

    compiler.compile().map_err(|e| {
        compiler.debug();
        e
    })?;

    if config.debug {
        compiler.debug_bin();
    }

    let _ = config.output.write(&compiler.bin);

    Ok(())
}
