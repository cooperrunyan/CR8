use anyhow::Result;
use sim::runner::Runner;

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let runner = Runner::from_argv()?;
    runner.run()?;

    Ok(())
}
