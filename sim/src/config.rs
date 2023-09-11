use std::time::Duration;
use std::{env, fs};

pub struct Config {
    pub bin: Vec<u8>,
    pub tickrate: Duration,
    pub debug: bool,
    pub step: bool,
}

impl Config {
    pub fn from_argv() -> Self {
        let args: Vec<_> = env::args().collect();

        let mut file = String::new();
        let mut tickrate = Duration::ZERO;
        let mut debug = false;
        let mut step = false;

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
                "-s" | "--step" => {
                    step = true;
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

        Self {
            bin,
            tickrate,
            debug,
            step,
        }
    }
}
