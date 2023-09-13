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

                    tickrate = parse_duration(&args[i + 1]);
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

fn parse_duration(inpt: &str) -> Duration {
    if let Some(hz) = inpt.strip_suffix("hz") {
        let (val, modif) = if let Some(as_khz) = hz.strip_suffix("k") {
            (as_khz.parse::<f64>(), 1_000)
        } else if let Some(as_mhz) = hz.strip_suffix("m") {
            (as_mhz.parse::<f64>(), 1_000_000)
        } else if let Some(as_ghz) = hz.strip_suffix("g") {
            (as_ghz.parse::<f64>(), 1_000_000_000)
        } else {
            (hz.parse::<f64>(), 1)
        };

        let val = val.expect("Invalid speed value");
        let per_sec = val * modif as f64;
        Duration::from_secs_f64(1.0 / per_sec)
    } else {
        panic!("Expected hz for clock speed");
    }
}
