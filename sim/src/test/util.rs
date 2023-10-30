use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use asm::compiler::{Compiler, Input};

use crate::cr8::CR8;
use crate::runner::Runner;

macro_rules! t {
    ($call:literal; $start:ident: $val:expr $(, $oth_start:ident: $oth_val:expr)* => $exp:ident: $tobe:expr $(, $oth_exp:ident: $oth_tobe:expr)* ) => {{

        let mut mov = String::new();

        mov.push_str(&format!("mov %{}, {}\n", Register::$start, $val));
         $(  mov.push_str(&format!("mov %{}, {}\n", Register::$oth_start, $oth_val));  )*

        let state = $crate::test::util::run_asm(format!("{mov}\n call {}", $call))?;

        assert!(state.reg[Register::$exp as usize] == $tobe);
        $( assert!(state.reg[Register::$oth_exp as usize] ==  $oth_tobe); )*


    }};

    ($call:literal => $exp:ident: $tobe:expr $(, $oth_exp:ident: $oth_tobe:expr)* ) => {{
        let state = $crate::test::util::run_asm($call.to_string())?;

        assert!(state.reg[Register::$exp as usize] == $tobe);
        $( assert!(state.reg[Register::$oth_exp as usize] ==  $oth_tobe); )*
    }};
}

pub fn run_asm(asm: String) -> Result<CR8> {
    let asm = format!(
        r#"
    #[use( std )]

    #[main]
    test:
    {asm}
        halt"#
    );

    let mut compiler = Compiler::new();
    compiler.push(Input::Raw(asm), Arc::new(PathBuf::from("test")))?;
    compiler.compile()?;
    let mut runner = Runner::new(&compiler.bin, Duration::ZERO, false);

    loop {
        let (_, should_continue) = runner.cycle()?;
        if !should_continue {
            break Ok(runner.cr8.into_inner().unwrap());
        }
    }
}
