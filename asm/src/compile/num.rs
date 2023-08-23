use std::num::ParseIntError;

pub fn try_read_num(raw: &str) -> Result<u64, ParseIntError> {
    let raw = raw.trim();

    if let Some(b) = raw.strip_prefix("0b") {
        u64::from_str_radix(b, 2)
    } else if let Some(h) = raw.strip_prefix("0x") {
        u64::from_str_radix(h, 16)
    } else {
        raw.parse()
    }
}
