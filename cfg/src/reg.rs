const A: u64 = 0x00;
const B: u64 = 0x01;
const C: u64 = 0x02;
const D: u64 = 0x03;
const Z: u64 = 0x04;
const L: u64 = 0x05;
const H: u64 = 0x06;
const F: u64 = 0x07;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    D,
    Z,
    L,
    H,
    F,
}

impl_for!(for Register, as u64, impl u8, u16, u32, usize);

impl TryFrom<u64> for Register {
    type Error = String;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            A => Ok(Self::A),
            B => Ok(Self::B),
            C => Ok(Self::C),
            D => Ok(Self::D),
            Z => Ok(Self::Z),
            L => Ok(Self::L),
            H => Ok(Self::H),
            F => Ok(Self::F),

            x => Err(format!("Invalid register: {x:#?}")),
        }
    }
}

impl_str!(for Register);

impl TryFrom<&str> for Register {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "z" => Ok(Self::Z),
            "l" => Ok(Self::L),
            "h" => Ok(Self::H),
            "f" => Ok(Self::F),

            x => Err(format!("Invalid register: {x:#?}")),
        }
    }
}
