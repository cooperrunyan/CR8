use std::time::Duration;
use std::{env, fs};

use anyhow::Result;

use super::Runner;

impl Runner {
    pub fn from_argv() -> Result<Self> {
        let args: Vec<_> = env::args().collect();

        let mut file = String::new();
        let mut tickrate = Duration::ZERO;
        let mut debug = false;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-i" | "--input" => {
                    if args.len() <= i + 1 {
                        break;
                    }
                    file = args[i + 1].to_string();
                }
                "-r" | "--tickrate" => {
                    if args.len() <= i + 1 {
                        break;
                    }
                    let tick = args[i + 1].parse::<u64>().unwrap();
                    tickrate = Duration::from_millis(tick);
                }
                "-d" | "--debug" => {
                    debug = true;
                }
                _ => {}
            }
        }

        if file.is_empty() {
            panic!("Expected input file");
        }

        let bin = match fs::read(file.clone()) {
            Ok(i) => i,
            Err(_) => panic!("Could not read input file"),
        };

        let mut runner = Self::new(tickrate, debug);
        runner.load(&bin)?;
        Ok(runner)
    }
}
