use anyhow::{bail, Result};
use path_clean::clean;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;

use crate::std::STD;

#[derive(Debug, Clone)]
pub struct Config {
    pub input: Input,
    pub output: Output,
}

#[derive(Debug, Clone)]
pub enum Input {
    Raw(String),
    File(String),
}

#[derive(Debug, Clone)]
pub enum Output {
    File(String),
    None,
}

impl Output {
    pub fn write(&self, bin: &[u8]) -> Result<(), io::Error> {
        match self {
            Output::None => Ok(()),
            Output::File(f) => {
                let mut options = OpenOptions::new();
                let mut file = options
                    .write(true)
                    .truncate(true)
                    .append(false)
                    .create(true)
                    .open(f)?;

                file.write_all(bin)
            }
        }
    }
}

impl Config {
    pub fn from_argv() -> Self {
        let mut input: Option<Input> = None;
        let mut output = Output::None;

        for (i, arg) in std::env::args().enumerate() {
            match arg.as_str() {
                "-i" | "--input" => {
                    if input.is_some() {
                        panic!("Attempted to set input flag twice");
                    }
                    input = Some(Input::File(std::env::args().nth(i + 1).unwrap_or_default()));
                }
                "-o" | "--output" => {
                    output = Output::File(std::env::args().nth(i + 1).unwrap_or_default());
                }
                _ => {}
            }
        }
        if input.is_none() {
            panic!("Did not specify input file")
        }
        let input = input.unwrap();

        Self { input, output }
    }
}

impl Input {
    pub fn source(
        self,
        from: Option<&PathBuf>,
        visited: Option<&Vec<Arc<PathBuf>>>,
    ) -> Result<(Option<String>, PathBuf)> {
        match self {
            Input::File(path) => {
                let pb = Arc::new(PathBuf::from(&path));
                if path.starts_with("std")
                    || path.starts_with("core")
                    || path.starts_with("prelude")
                {
                    if let Some(visited) = visited {
                        for included in visited {
                            if included == &pb {
                                return Ok((None, pb.to_path_buf()));
                            }
                        }
                    }
                    if let Some(content) = STD.get(&path) {
                        Ok((Some(content.to_string()), path.into()))
                    } else {
                        bail!("No std module: {path}");
                    }
                } else {
                    let pb = pb.to_path_buf();
                    let real = if pb.exists() && pb.is_file() {
                        pb.to_path_buf()
                    } else {
                        let possibilities = match from {
                            Some(f) => vec![
                                f.parent().unwrap_or(f).join(&pb),
                                f.parent().unwrap_or(f).join(&pb).with_extension("asm"),
                                f.parent().unwrap_or(f).join(&pb).join("mod.asm"),
                                f.parent().unwrap_or(f).join(&pb).join("main.asm"),
                                pb.with_extension("asm"),
                                pb.join("main.asm"),
                                pb.join("mod.asm"),
                                pb,
                            ],
                            None => vec![
                                pb.with_extension("asm"),
                                pb.join("main.asm"),
                                pb.join("mod.asm"),
                                pb,
                            ],
                        };

                        let mut found = None;

                        for possible in possibilities.iter() {
                            if possible.exists() && possible.is_file() {
                                found = Some(possible.to_owned());
                                break;
                            }
                        }

                        match found {
                            Some(p) => clean(p.as_path()),
                            None => {
                                let attempted = possibilities
                                    .into_iter()
                                    .map(|p| clean(p.as_path()))
                                    .collect::<Vec<_>>();
                                bail!("No file {path:#?} found in any of {:#?}", attempted);
                            }
                        }
                    };

                    match fs::read_to_string(&real) {
                        Ok(file) => Ok((Some(file), real.to_path_buf())),
                        Err(_) => bail!("Failed to read {real:?}"),
                    }
                }
            }

            Input::Raw(s) => Ok((Some(s), "raw".into())),
        }
    }
}
