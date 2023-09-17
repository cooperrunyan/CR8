use anyhow::Result;
fn main() -> Result<()> {
    use env_logger::Env;
    use sim::runner::Runner;
    use std::thread;

    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .format_target(false)
        .parse_env(Env::new().default_filter_or("sim=info"))
        .init();

    let mut runner = Runner::from_argv()?;

    loop {
        let ticks = runner.cycle()?;
        thread::sleep(runner.tickrate * ticks as u32);
    }
}
