pub const A: u64 = 0x00;
pub const B: u64 = 0x01;
pub const C: u64 = 0x02;
pub const D: u64 = 0x03;
pub const Z: u64 = 0x04;
pub const L: u64 = 0x05;
pub const H: u64 = 0x06;
pub const F: u64 = 0x07;

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

impl From<u64> for Register {
    fn from(value: u64) -> Self {
        match value {
            A => Self::A,
            B => Self::B,
            C => Self::C,
            D => Self::D,
            Z => Self::Z,
            L => Self::L,
            H => Self::H,
            F => Self::F,

            _ => panic!("Invalid register: {value}"),
        }
    }
}

uint!(Register);

impl From<&str> for Register {
    fn from(value: &str) -> Self {
        match value {
            "%ax" => Self::A,
            "%bx" => Self::B,
            "%cx" => Self::C,
            "%dx" => Self::D,
            "%zx" => Self::Z,
            "%lx" => Self::L,
            "%hx" => Self::H,
            "%fx" => Self::F,

            x => panic!("Invalid register name: {x}"),
        }
    }
}
