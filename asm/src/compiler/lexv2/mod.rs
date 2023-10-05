mod directive;
mod expr;
mod lexable;
mod node;
mod tree;

pub use directive::*;
pub use expr::*;
pub use lexable::*;
pub use node::*;
pub use tree::*;

//////////////////////////////////

#[cfg(test)]
mod test {
    use super::{Directive, LexError, Lexable, NodeTree};

    #[test]
    fn directive<'s>() -> Result<(), LexError> {
        let _ = Directive::lex(
            r#"#[macro] jnz: {
                ($addr: imm16, $if: imm8 | reg) => {
                    ldhl $addr
                    jnz $if
                }
                ($addr: imm8, $if: imm8 | reg) => {
                    jnz $if
                }
            }"#,
        )?;

        let _ = Directive::lex("#[static(HELLO: 0xFF00)]")?;
        let _ = Directive::lex("#[static(HELLO: 2)]")?;
        let _ = Directive::lex("#[static(HELLO: 0b1001)]")?;

        let _ = Directive::lex("#[use(\"./std/test.asm\")]")?;
        let _ = Directive::lex("#[use(std::test)]")?;

        let _ = Directive::lex("#[dyn(TEST: 4)]")?;
        let _ = Directive::lex("#[dyn(&0xC000)]")?;

        let _ = Directive::lex("#[boot] main: mov %a, %b")?;

        let _ = Directive::lex(
            r#"#[explicit(TEST)] {
                    0x00, 0x00, 0x00
                }"#,
        )?;
        Ok(())
    }

    #[test]
    fn d<'s>() -> Result<(), LexError> {
        let f = r#"
#[macro] nand: {
    ($into: reg, $rhs: imm8 | reg) => {
        and $into, $rhs
        not $into
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        nand $inl, $frl
        nand $inh, $frh
    }
}

#[macro] not: {
    ($into: reg) => {
        nor $into, $into
    }
    ($inl: reg, $inh: reg) => {
        not $inl
        not $inh
    }
}

#[macro] xnor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $into
        nor $into, $rhs
        and %f, $rhs
        or $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xnor $inl, $frl
        xnor $inh, $frh
    }
}


#[macro] xor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $rhs
        or %f, $into
        nand $into, $rhs
        and $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xor $inl, $frl
        xor $inh, $frh
    }
}

#[static(ROM: 0x0000)]
#[static(BRAM: 0x8000)]
#[static(GPRAM: 0xC000)]
#[static(STACK: 0xFC00)]
#[static(STACK_END: 0xFEFF)]

#[static(PSR0: 0xFF00)]
#[static(PSR1: 0xFF01)]
#[static(PSR2: 0xFF02)]
#[static(PSR3: 0xFF03)]
#[static(PSR4: 0xFF04)]
#[static(PSR5: 0xFF05)]
#[static(PSR6: 0xFF06)]
#[static(PSR7: 0xFF07)]
#[static(PSR8: 0xFF08)]
#[static(PSR9: 0xFF09)]

#[static(CTRL: 0x00)]
#[static(SIGPING: 0x00)]
#[static(SIGHALT: 0x01)]
#[static(SIGDBG: 0x02)]
#[static(SIGBRKPT: 0x03)]

#[static(WAIT: 0x2000)]
#[static(OFFSET: 0x0400)]

#[boot]
main:
    mb 0x01
    jmp [hello]

hello:
    mov %a, %b, [HELLO]
    mov %c, %d, [BRAM]
    sw [PSR0], [HELLO_SZL]
    sw [PSR1], [HELLO_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM], [BRAM + HELLO_SZ]

    jmp [world]

world:
    mov %a, %b, [WORLD]
    mov %c, %d, [BRAM + OFFSET]
    sw [PSR0], [WORLD_SZL]
    sw [PSR1], [WORLD_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM + OFFSET], [BRAM + OFFSET + WORLD_SZ]

    jmp [hello]


        "#;

        let cc = NodeTree::lex(f).unwrap();

        dbg!(&cc);

        Ok(())
    }
}
