use std::env;
use std::sync::Arc;

use anyhow::Result;

use asm::compiler;

use super::Runner;

impl Runner {
    pub fn from_argv() -> Result<Self> {
        let (file, tickrate) = Self::read_argv()?;
        let bin = Self::jit(file)?;

        Ok(Self::new(&bin, tickrate))
    }

    pub fn jit(path: String) -> Result<Vec<u8>> {
        let config = compiler::Config {
            input: compiler::Input::File(path),
            output: compiler::Output::None,
        };
        let mut compiler = compiler::Compiler::new();

        compiler.push(config.input, Arc::new(env::current_dir().unwrap()))?;

        let bin = compiler.compile()?;
        Ok(bin)
    }
}
