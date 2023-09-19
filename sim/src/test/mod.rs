use anyhow::Result;
use asm::reg::Register;

#[macro_use]
mod util;

#[test]
fn lsh() -> Result<()> {
    t!("lsh"; A: 0b00010000, B: 2  =>  Z: 0b01000000);
    t!("lsh"; A: 0b11111001, B: 1  =>  Z: 0b11110010);

    Ok(())
}

#[test]
fn lsa() -> Result<()> {
    t!("lsa"; A: 0b10111001, B: 1  =>  Z: 0b11110010);
    t!("lsa"; A: 0b00000001, B: 1  =>  Z: 0b00000010);

    Ok(())
}

#[test]
fn lrt() -> Result<()> {
    t!("lrt"; A: 0b11101011, B: 1  =>  Z: 0b11010111);
    t!("lrt"; A: 0b01101011, B: 1  =>  Z: 0b11010110);

    Ok(())
}

#[test]
fn lsh16() -> Result<()> {
    t!("lsh16"; B: 0b00010000, A: 0b00010000, C: 4  =>  B: 0b00000001, A: 0b00000000);
    t!("lsh16"; B: 0b11111001, A: 0b11111001, C: 4  =>  B: 0b10011111, A: 0b10010000);

    Ok(())
}

#[test]
fn lsa16() -> Result<()> {
    t!("lsa16"; B: 0b10111001, A: 0b10111001, C: 4  =>  B: 0b10011011, A: 0b10010000);
    t!("lsa16"; B: 0b00001101, A: 0b00000001, C: 4  =>  B: 0b01010000, A: 0b00010000);

    Ok(())
}

#[test]
fn rrt() -> Result<()> {
    t!("rrt"; A: 0b11010111, B: 1  =>  Z: 0b11101011);
    t!("rrt"; A: 0b11010110, B: 1  =>  Z: 0b01101011);

    Ok(())
}

#[test]
fn rsh() -> Result<()> {
    t!("rsh"; A: 0b01000000, B: 2  =>  Z: 0b00010000);
    t!("rsh"; A: 0b11110010, B: 1  =>  Z: 0b01111001);

    Ok(())
}

#[test]
fn mul() -> Result<()> {
    let i0: u16 = 24;
    let i1: u16 = 19;
    let o0 = i0.overflowing_mul(i1).0 as u8;
    let o1 = (i0.overflowing_mul(i1).0 >> 8) as u8;
    let i0 = 24 as u8;
    let i1 = 19 as u8;
    t!("mul"; A: i0, B: i1  =>  D: o1, Z: o0);

    let i0: u16 = 7;
    let i1: u16 = 99;
    let o0 = i0.overflowing_mul(i1).0 as u8;
    let o1 = (i0.overflowing_mul(i1).0 >> 8) as u8;
    let i0 = 7 as u8;
    let i1 = 99 as u8;
    t!("mul"; A: i0, B: i1  =>  D: o1, Z: o0);

    Ok(())
}

#[test]
fn mulip() -> Result<()> {
    let i0: u16 = 24;
    let i1: u16 = 19;
    let o0 = i0.overflowing_mul(i1).0 as u8;
    let o1 = (i0.overflowing_mul(i1).0 >> 8) as u8;
    let i0 = 24 as u8;
    let i1 = 19 as u8;
    t!("mulip"; A: i0, B: i1  =>  B: o1, A: o0);

    let i0: u16 = 7;
    let i1: u16 = 99;
    let o0 = i0.overflowing_mul(i1).0 as u8;
    let o1 = (i0.overflowing_mul(i1).0 >> 8) as u8;
    let i0 = 7 as u8;
    let i1 = 99 as u8;
    t!("mulip"; A: i0, B: i1  =>  B: o1, A: o0);

    Ok(())
}

#[test]
fn mul16() -> Result<()> {
    macro_rules! test_mul16 {
        ($ab:expr, $cd:expr) => {{
            let ab: u16 = $ab;
            let cd: u16 = $cd;
            let out = (ab as u32).overflowing_mul(cd as u32).0;
            let a_in = ab as u8;
            let b_in = (ab >> 8) as u8;
            let c_in = cd as u8;
            let d_in = (cd >> 8) as u8;
            let a_out = out as u8;
            let b_out = (out >> 8) as u8;
            let c_out = (out >> 16) as u8;
            let d_out = (out >> 24) as u8;
            t!("mul16"; A: a_in, B: b_in, C: c_in, D: d_in  =>  A: a_out, B: b_out, C: c_out, D: d_out)
        }}
    }

    test_mul16!(5201, 19246);
    test_mul16!(1975, 9276);

    Ok(())
}
