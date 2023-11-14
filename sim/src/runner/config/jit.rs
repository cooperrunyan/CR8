use std::env;
use std::sync::Arc;

use anyhow::Result;

use asm::compiler;

use super::Runner;

impl Runner {
    pub fn from_argv() -> Result<Self> {
        let (file, tickrate, debug) = Self::read_argv()?;
        let bin = Self::jit(String::from_utf8(file)?)?;

        Ok(Self::new(&bin, tickrate, debug))
    }

    pub fn jit(file: String) -> Result<Vec<u8>> {
        let config = compiler::Config {
            input: compiler::Input::Raw(file),
            output: compiler::Output::default(),
            micro: false,
            debug: false,
        };
        let mut compiler = compiler::Compiler::new();

        compiler.push(config.input, Arc::new(env::current_dir().unwrap()))?;

        compiler.compile()?;
        Ok(compiler.bin)
    }
}
