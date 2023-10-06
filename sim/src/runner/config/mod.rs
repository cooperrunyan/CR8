use anyhow::{bail, Result};
use std::{env, time::Duration};

use super::Runner;

#[cfg(feature = "jit")]
mod jit;

impl Runner {
    #[cfg(not(feature = "jit"))]
    pub fn from_argv() -> Result<Self> {
        use std::fs;

        let (file, tickrate, debug) = Self::read_argv()?;

        let bin = match fs::read(file.clone()) {
            Ok(i) => i,
            Err(_) => bail!("Could not read input file"),
        };

        Ok(Self::new(&bin, tickrate, debug))
    }

    pub fn read_argv() -> Result<(String, Duration, bool)> {
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

                    tickrate = Self::parse_duration(&args[i + 1])?;
                }
                "--dbg" => {
                    debug = true;
                }
                _ => {}
            }
        }

        if file.is_empty() {
            bail!("Expected input file");
        }

        Ok((file, tickrate, debug))
    }

    pub fn parse_duration(inpt: &str) -> Result<Duration> {
        if let Some(hz) = inpt.strip_suffix("hz") {
            let (val, modif) = if let Some(as_khz) = hz.strip_suffix('k') {
                (as_khz.parse::<f64>(), 1_000)
            } else if let Some(as_mhz) = hz.strip_suffix('m') {
                (as_mhz.parse::<f64>(), 1_000_000)
            } else if let Some(as_ghz) = hz.strip_suffix('g') {
                (as_ghz.parse::<f64>(), 1_000_000_000)
            } else {
                (hz.parse::<f64>(), 1)
            };

            let val = val.expect("Invalid speed value");
            let per_sec = val * modif as f64;
            Ok(Duration::from_secs_f64(1.0 / per_sec))
        } else {
            bail!("Expected hz for clock speed");
        }
    }
}
