use cfg::reg::Register;

macro_rules! test {
    ($asm:literal) => {{
        use crate::cr8::{CR8Config, CR8};
        use asm;
        use std::path::PathBuf;

        let cfg = asm::Config {
            literal: $asm.to_string(),
            input: PathBuf::from(""),
            output: PathBuf::from(""),
        };

        let bin = asm::compile(cfg);
        let bin = bin.bytes().collect::<Vec<_>>();
        let mut cr8 = CR8::new(CR8Config::builder().tick_rate(0).mem(bin).build());
        cr8.run();
        cr8
    }};
}

#[test]
fn adc() {
    let state = test!(
        r#"
  mov %ax, 12
  mov %bx, 9
  mov %cx, 8
  add %ax, %b
  add %cx, %a
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 21);
    assert_eq!(state.reg[Register::B as usize], 9);
    assert_eq!(state.reg[Register::C as usize], 29);
}

#[test]
fn sbb() {
    let state = test!(
        r#"
  mov %ax, 12
  mov %bx, 9
  sbb %ax, %bx
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
  mov %ax, 12
  mov %bx, %ax
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
  mov %ax, 20
  sw %ax, 0x00, 0xFD
  lw %bx, 0x00, 0xFD
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
  mov %ax, 9
  mov %bx, 4
  mov %cx, 3
  mov %dx, 8
  push %ax
  push %bx
  push %cx
  push %dx
  mov %ax, 0
  mov %bx, 0
  mov %cx, 0
  mov %dx, 0
  pop %dx
  pop %cx
  pop %bx
  pop %ax
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
  mov %ax, 9
  mov %bx, 12
  cmp %ax, %bx
  halt
"#
    );

    assert_eq!(state.reg[Register::F as usize] & 0b0011, 0b0001);
}

#[test]
fn cmp_eq() {
    let state = test!(
        r#"
  mov %ax, 12
  mov %bx, 12
  cmp %ax, %bx
  halt
"#
    );

    assert_eq!(state.reg[Register::F as usize] & 0b0011, 0b0010);
}

#[test]
fn and() {
    let state = test!(
        r#"
  mov %ax, 0b00111100
  mov %bx, 0b11001100
  and %ax, %bx
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b00001100);
}

#[test]
fn or() {
    let state = test!(
        r#"
  mov %ax, 0b00111100
  mov %bx, 0b11001100
  or %ax, %bx
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b11111100);
}

#[test]
fn nor() {
    let state = test!(
        r#"
  mov %ax, 0b00111100
  mov %bx, 0b11001100
  nor %ax, %bx
  halt
"#
    );

    assert_eq!(state.reg[Register::A as usize], 0b00000011);
}
