use anyhow::Result;
use asm::compiler::{Compiler, Config};

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .init();

    let config = Config::from_argv();
    let mut compiler = Compiler::new();

    compiler.push(config.input)?;

    let _ = config.output.write(&compiler.compile()?);

    Ok(())
}
