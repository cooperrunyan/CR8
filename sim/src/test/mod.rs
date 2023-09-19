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
