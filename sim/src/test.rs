use anyhow::Result;
use asm::reg::Register;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use asm::compiler::{Compiler, Input};

use crate::cr8::CR8;
use crate::runner::Runner;

macro_rules! run {
    ($asm:literal) => {
        _test(format!($asm))?
    };
}

macro_rules! assert_reg {
    ($state:ident, $reg:ident, $val:expr) => {
        assert_eq!($state.reg[Register::$reg as usize], $val);
    };
}

#[test]
fn lsh() -> Result<()> {
    let a = 0b00010000;
    let b = 2;

    let state = run! {r#"
    #use "<std>/macro"
    #use "<std>/math/shift/lsh"
    #init {{
        mov %a, {a}
        mov %b, {b}
        call [lshl]
        halt
    }}"#};

    assert_reg!(state, Z, a << b);

    Ok(())
}

fn _test(asm: String) -> Result<CR8> {
    let mut compiler = Compiler::new();
    compiler.push(Input::Raw(asm), Arc::new(PathBuf::from("test")))?;
    let bin = compiler.compile()?;
    let mut runner = Runner::new(&bin, Duration::ZERO);

    loop {
        let (_, should_continue) = runner.cycle()?;
        if !should_continue {
            break Ok(runner.cr8.into_inner().unwrap());
        }
    }
}
