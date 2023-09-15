use anyhow::Result;

use super::Runner;

impl Runner {
    pub fn run(mut self) -> Result<()> {
        use std::thread;

        loop {
            let ticks = self.cycle()?;
            thread::sleep(self.tickrate * ticks as u32);
        }
    }
}
