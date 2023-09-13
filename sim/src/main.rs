use anyhow::Result;
use sim::runner::Runner;

fn main() -> Result<()> {
    let mut runner = Runner::from_argv()?;
    runner.run()?;

    runner.debug()?;

    Ok(())
}
