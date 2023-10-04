use path_clean::clean;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;

use failure::Fail;

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
    pub fn write(&self, bin: &Vec<u8>) -> Result<(), io::Error> {
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
                    input = Some(Input::File(
                        std::env::args().nth(i + 1).unwrap_or_default().into(),
                    ));
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

#[derive(Fail, Debug)]
pub enum SourceInputError {
    #[fail(display = "import of unknown std module: {}", _0)]
    NoStdModule(String),

    #[fail(display = "file not found in any of: \n{:#?}", _0)]
    NoFile(Vec<PathBuf>),

    #[fail(display = "failed to read file: {:#?}", _0)]
    ReadError(PathBuf),
}

impl Input {
    pub fn source(
        self,
        from: Option<&PathBuf>,
        visited: Option<&Vec<PathBuf>>,
    ) -> Result<(Option<String>, PathBuf), SourceInputError> {
        match self {
            Input::File(path) => {
                let pb = PathBuf::from(&path);
                if path.starts_with("std") {
                    if let Some(visited) = visited {
                        for included in visited {
                            if included == &pb {
                                return Ok((None, pb));
                            }
                        }
                    }
                    Ok((Some(String::from("")), path.into()))
                    // if let Some(content) = STD.get(&path) {
                    //     Ok((Some(content.to_string()), path.into()))
                    // } else {
                    //     Err(SourceInputError::NoStdModule(path))
                    // }
                } else {
                    let real = if pb.exists() && pb.is_file() {
                        pb
                    } else {
                        let possibilities = match from {
                            Some(f) => vec![
                                f.parent().unwrap_or(&f).join(&pb),
                                f.parent().unwrap_or(&f).join(&pb).with_extension("asm"),
                                f.parent().unwrap_or(&f).join(&pb).join("mod.asm"),
                                f.parent().unwrap_or(&f).join(&pb).join("main.asm"),
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
                                return Err(SourceInputError::NoFile(attempted));
                            }
                        }
                    };

                    match fs::read_to_string(&real) {
                        Ok(file) => Ok((Some(file), real)),
                        Err(_) => Err(SourceInputError::ReadError(real)),
                    }
                }
            }

            Input::Raw(s) => Ok((Some(s), "raw".into())),
        }
    }
}
