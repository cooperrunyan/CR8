use anyhow::{bail, Result};

use super::lex::{expect_complete, Lexable, Micro, Pragma};
use super::Input;

pub fn compile(input: Input) -> Result<Vec<u8>> {
    let (buf, _) = input.source(None, None)?;
    let buf = buf.unwrap_or_default();
    #[allow(unused_mut)]
    let mut bin = vec![];

    let (prag, buf) = Pragma::lex(&buf)?;

    if prag != Pragma::Micro {
        bail!("Expected #![micro] at the beginning of a microcode file");
    }

    let (micro, buf) = Micro::lex(buf)?;

    expect_complete(buf)?;

    dbg!(micro);

    Ok(bin)
}
