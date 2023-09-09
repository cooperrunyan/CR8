pub const ROM: u16 = 0x0000;
pub const VRAM: u16 = 0x8000;
pub const GPRAM: u16 = 0xC000;
pub const STACK: u16 = 0xFC00;
pub const STACK_END: u16 = 0xFEFF;
pub const STACK_POINTER: u16 = 0xFFFC;
pub const PROGRAM_COUNTER: u16 = 0xFFFE;

pub const DEV_CONTROL: u8 = 0x00;
pub const SIGNOP: u8 = 0x00;
pub const SIGHALT: u8 = 0x01;
pub const SIGPEEK: u8 = 0x02;
pub const SIGDBG: u8 = 0x03;
