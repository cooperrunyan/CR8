use anyhow::{bail, Result};
use std::fs;
use std::{env, time::Duration};

use super::Runner;

#[cfg(feature = "jit")]
mod jit;

impl Runner {
    #[cfg(not(feature = "jit"))]
    pub fn from_argv() -> Result<Self> {
        let (bytes, tickrate, debug) = Self::read_argv()?;

        Ok(Self::new(&bytes, tickrate, debug))
    }

    pub fn read_argv() -> Result<(Vec<u8>, Duration, bool)> {
        let args: Vec<_> = env::args().collect();

        let mut file: Option<Vec<u8>> = None;
        let mut tickrate = Duration::ZERO;
        let mut debug = false;

        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "-f" => {
                    if args.len() <= i + 1 {
                        break;
                    }
                    file = Some(fs::read(args[i + 1].clone())?);
                }
                "-x" => {
                    if args.len() <= i + 1 {
                        break;
                    }
                    file = Some(args[i + 1].clone().into_bytes());
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

        if file.is_none() {
            bail!("Expected input file");
        }

        Ok((file.unwrap(), tickrate, debug))
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
