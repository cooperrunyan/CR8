use asm::reg::Register;

macro_rules! test {
    ($asm:literal) => {{
        use crate::cr8::{CR8Config, CR8};
        use asm;
        use std::path::PathBuf;

        let cfg = asm::Config {
            literal: $asm.to_string(),
            input: String::from(""),
            output: PathBuf::from(""),
        };

        let mut bin = asm::compile(cfg);
        bin.push(0xFF);
        let mut cr8 = CR8::new(CR8Config::builder().tick_rate(0).mem(bin).build());
        let _ = cr8.run();
        cr8
    }};
}

#[test]
fn adc() {
    let state = test!(
        r#"
  mov %a, 9
  mov %b, 9
  mov %c, 8
  adc %a, %b
"#
    );

    assert_eq!(state.reg[Register::A as usize], 21);
    assert_eq!(state.reg[Register::B as usize], 9);
    assert_eq!(state.reg[Register::C as usize], 8);
}

#[test]
fn sbb() {
    let state = test!(
        r#"
  mov %a, 12
  mov %b, 9
  sbb %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 3);
    assert_eq!(state.reg[Register::B as usize], 9);
}

#[test]
fn mov() {
    let state = test!(
        r#"
  mov %a, 12
  mov %b, %a
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 12);
    assert_eq!(state.reg[Register::B as usize], 12);
}

#[test]
fn lwsw() {
    let state = test!(
        r#"
  mov %a, 20
  sw %a, 0x00, 0xFD
  lw %b, 0x00, 0xFD
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 20);
    assert_eq!(state.reg[Register::B as usize], 20);
    assert_eq!(state.mem[0xFD00_usize], 20);
}

#[test]
fn stack() {
    let state = test!(
        r#"
  mov %a, 9
  mov %b, 4
  mov %c, 3
  mov %d, 8
  push %a
  push %b
  push %c
  push %d
  mov %a, 0
  mov %b, 0
  mov %c, 0
  mov %d, 0
  pop %d
  pop %c
  pop %b
  pop %a
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 9);
    assert_eq!(state.reg[Register::B as usize], 4);
    assert_eq!(state.reg[Register::C as usize], 3);
    assert_eq!(state.reg[Register::D as usize], 8);
}

#[test]
fn cmp_lt() {
    let state = test!(
        r#"
  mov %a, 9
  mov %b, 12
  cmp %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::F as usize] & 0b0011, 0b0001);
}

#[test]
fn cmp_eq() {
    let state = test!(
        r#"
  mov %a, 12
  mov %b, 12
  cmp %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::F as usize] & 0b0011, 0b0010);
}

#[test]
fn and() {
    let state = test!(
        r#"
  mov %a, 0b00111100
  mov %b, 0b11001100
  and %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b00001100);
}

#[test]
fn or() {
    let state = test!(
        r#"
  mov %a, 0b00111100
  mov %b, 0b11001100
  or %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b11111100);
}

#[test]
fn nor() {
    let state = test!(
        r#"
  mov %a, 0b00111100
  mov %b, 0b11001100
  nor %a, %b
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b00000011);
}
