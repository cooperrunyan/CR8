use anyhow::Result;
use asm::reg::Register;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use asm::compiler::{Compiler, Input};

use crate::cr8::CR8;
use crate::runner::Runner;

macro_rules! t {
    ($call:literal; $start:ident: $val:expr $(, $oth_start:ident: $oth_val:expr)* => $exp:ident: $tobe:expr $(, $oth_exp:ident: $oth_tobe:expr)* ) => {
        crate::test::util::run_test($call.to_string(), vec![
            (Register::$start, $val),
            $( (Register::$oth_start, $oth_val), )*
        ], vec![
            (Register::$exp, $tobe),
            $( (Register::$oth_exp, $oth_tobe), )*
        ])?
    }
}

fn run_asm(asm: String) -> Result<CR8> {
    let asm = format!(
        r#"
    #[use( std )]

    #[boot]
    test:
    {asm}
        halt"#
    );

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

pub(super) fn run_test(
    call: String,
    start: Vec<(Register, u8)>,
    expect: Vec<(Register, u8)>,
) -> Result<()> {
    let mut mov = String::new();
    for (r, v) in start {
        mov.push_str(&format!("mov %{r}, {v}\n"));
    }

    let state = run_asm(format!("{mov}\n call [{call}]"))?;
    for (r, exp) in expect {
        assert_eq!(state.reg[r as usize], exp);
    }

    Ok(())
}
