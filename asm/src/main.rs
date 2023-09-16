use anyhow::Result;
use asm::compiler::{Compiler, Config};

use env_logger::Env;

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .parse_env(Env::new().default_filter_or("asm=info"))
        .init();

    let config = Config::from_argv();
    let mut compiler = Compiler::new();

    compiler.push(config.input)?;

    let _ = config.output.write(&compiler.compile()?);

    Ok(())
}
