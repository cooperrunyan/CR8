use cfg::reg::Register;

mod expr;
mod num;
mod resolve_macros;
mod scan;
mod section;

#[derive(Debug)]
enum Arg {
    Byte(u8),
    Register(Register),
}

pub use scan::{scan, SymbolType};
pub use section::compile_section as section;
