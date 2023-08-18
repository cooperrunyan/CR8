pub const ROM: u16 = 0x0000;
pub const VRAM: u16 = 0x8000;
pub const GP_RAM: u16 = 0xC000;
pub const STACK: u16 = 0xFC00;
pub const STACK_END: u16 = 0xFEFF;
pub const STACK_POINTER: u16 = 0xFFFC;
pub const PROGRAM_COUNTER: u16 = 0xFFFE;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    Byte,
    Word,
    Double,
}

impl From<&str> for Size {
    fn from(value: &str) -> Self {
        match value.trim() {
            "byte" => Self::Byte,
            "word" => Self::Word,
            "dble" => Self::Double,
            _ => panic!("Invalid size type"),
        }
    }
}

impl Size {
    pub fn val(&self, s: &str) -> SizedValue {
        match self {
            &Size::Byte => SizedValue::Byte(util::parse_num::<u64>(s) as u8),
            &Size::Word => SizedValue::Word(util::parse_num::<u64>(s) as u16),
            &Size::Double => SizedValue::Double(util::parse_num::<u64>(s) as u32),
        }
    }
    pub fn val_u(&self, s: &u64) -> SizedValue {
        match self {
            &Size::Byte => SizedValue::Byte(s.clone() as u8),
            &Size::Word => SizedValue::Word(s.clone() as u16),
            &Size::Double => SizedValue::Double(s.clone() as u32),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SizedValue {
    Byte(u8),
    Word(u16),
    Double(u32),
}

impl SizedValue {
    pub fn raw(&self) -> u64 {
        match self {
            &Self::Byte(b) => b as u64,
            &Self::Word(w) => w as u64,
            &Self::Double(d) => d as u64,
        }
    }
}

impl Into<Size> for SizedValue {
    fn into(self) -> Size {
        match self {
            Self::Byte(_) => Size::Byte,
            Self::Word(_) => Size::Word,
            Self::Double(_) => Size::Double,
        }
    }
}
