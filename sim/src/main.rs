use anyhow::Result;
use env_logger::Env;
use sim::runner::Runner;

fn main() -> Result<()> {
    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .parse_env(Env::new().default_filter_or("sim=info"))
        .init();

    let runner = Runner::from_argv()?;
    runner.run()?;

    Ok(())
}
