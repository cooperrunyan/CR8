pub const A: u8 = 0;
pub const B: u8 = 1;
pub const C: u8 = 2;
pub const D: u8 = 3;
pub const Z: u8 = 4;
pub const L: u8 = 5;
pub const H: u8 = 6;
pub const F: u8 = 7;

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

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::A,
            0x01 => Self::B,
            0x02 => Self::C,
            0x03 => Self::D,
            0x04 => Self::Z,
            0x05 => Self::L,
            0x06 => Self::H,
            0x07 => Self::F,

            _ => panic!(),
        }
    }
}

impl Into<u8> for Register {
    fn into(self) -> u8 {
        match self {
            Self::A => 0x00,
            Self::B => 0x01,
            Self::C => 0x02,
            Self::D => 0x03,
            Self::Z => 0x04,
            Self::L => 0x05,
            Self::H => 0x06,
            Self::F => 0x07,
        }
    }
}

impl From<usize> for Register {
    fn from(value: usize) -> Self {
        match value {
            0x00 => Self::A,
            0x01 => Self::B,
            0x02 => Self::C,
            0x03 => Self::D,
            0x04 => Self::Z,
            0x05 => Self::L,
            0x06 => Self::H,
            0x07 => Self::F,

            _ => panic!(),
        }
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        match self {
            Self::A => 0x00,
            Self::B => 0x01,
            Self::C => 0x02,
            Self::D => 0x03,
            Self::Z => 0x04,
            Self::L => 0x05,
            Self::H => 0x06,
            Self::F => 0x07,
        }
    }
}
