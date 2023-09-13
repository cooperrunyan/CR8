use anyhow::Result;
use sim::runner::Runner;

fn main() -> Result<()> {
    let runner = Runner::from_argv()?;
    runner.run()?;

    Ok(())
}
