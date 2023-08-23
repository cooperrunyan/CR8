use cfg::reg::Register;

macro_rules! test {
    ($asm:literal) => {{
        use crate::{cr8::CR8, exec};
        use asm;

        let mut bin = asm::compile($asm);
        bin.append(&mut vec![255]);
        let state = exec::exec(bin, CR8::new().speed(0));
        state
    }};
}

#[test]
fn test_add() {
    let state = test!(
        r#"
main:
  mov %a, 12
  mov %b, 9
  mov %c, 8
  add %a, %b
  add %c, %a
"#
    );

    assert_eq!(state.reg[Register::A as usize], 21);
    assert_eq!(state.reg[Register::B as usize], 9);
    assert_eq!(state.reg[Register::C as usize], 29);
}
