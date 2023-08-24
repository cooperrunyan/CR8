use std::{collections::HashMap, fs, path::PathBuf};

use super::{num::try_read_num, resolve_macros};
use cfg::mem;

pub fn scan(source: &str) -> Ctx {
    let mut ctx = Ctx::default();

    macro_rules! defb {
        ($val:expr, $name:literal) => {
            ctx.symbols
                .insert($name.to_string(), SymbolType::StaticByte($val));
        };
    }

    macro_rules! defw {
        ($val:expr, $name:literal) => {
            ctx.symbols
                .insert($name.to_string(), SymbolType::StaticWord($val));
        };
    }

    defw!(mem::ROM, "ROM");
    defw!(mem::VRAM, "VRAM");
    defw!(mem::GPRAM, "GPRAM");
    defw!(mem::STACK, "STACK");
    defw!(mem::STACK_END, "STACK_END");
    defw!(mem::STACK_POINTER, "STACK_POINTER");
    defw!(mem::PROGRAM_COUNTER, "PROGRAM_COUNTER");
    defb!(mem::DEV_CONTROL, "DEV_CONTROL");
    defb!(mem::SIGNOP, "SIGNOP");
    defb!(mem::SIGHALT, "SIGHALT");
    defb!(mem::SIGDBG, "SIGDBG");

    let builtin = include_bytes!("../std.asm");

    let _ = scan_with_ctx(
        String::from_utf8(builtin.to_vec()).unwrap().as_str(),
        &mut ctx,
    );

    let global = scan_with_ctx(source, &mut ctx);

    ctx.sections.insert(0, ("".to_string(), global));

    ctx
}

fn scan_with_ctx(source: &str, ctx: &mut Ctx) -> String {
    let mut out = String::new();

    let mut skip = 0;

    for (i, mut line) in source.lines().enumerate() {
        if skip != 0 {
            skip -= 1;
            continue;
        }

        if let Some(comm_ind) = line.find('#') {
            line = &line[0..comm_ind].trim_end();
        };

        if line.is_empty() {
            continue;
        }

        match line.trim_start().strip_prefix('@') {
            Some(l) => {
                if l == "macro" {
                    let mut mac = String::new();
                    let lines = source.lines().collect::<Vec<_>>();
                    for j in (i + 1)..lines.len() {
                        let current = lines.get(j).unwrap();
                        if !current.starts_with(' ') && !current.ends_with(':') {
                            break;
                        }
                        skip += 1;
                        let mut current = current.trim();

                        if let Some(comm_ind) = current.find('#') {
                            current = &current[0..comm_ind].trim();
                        };

                        if current.is_empty() {
                            continue;
                        }
                        mac.push('\n');
                        mac.push_str(current);
                    }

                    if mac.is_empty() {
                        panic!("Expected macro after @macro directive");
                    }

                    let (macro_header, macro_body) = mac.split_at(mac.find(':').unwrap_or(0));
                    let macro_header = macro_header.trim();
                    let (macro_name, macro_args) =
                        macro_header.split_at(macro_header.find(' ').unwrap_or(macro_header.len()));
                    let macro_name = macro_name.trim();

                    let args = macro_args
                        .split(',')
                         .map(|arg| arg.trim()).filter(|a| !a.is_empty())
                        .map(|arg| {
                            if !arg.starts_with('$') {
                                panic!("Macro arguments should start with '$'. '{arg}' in '{macro_args}'")
                            } else {
                                arg.to_string()
                            }
                        })
                        .collect::<Vec<_>>();

                    if ctx.macros.contains_key(macro_name) {
                        panic!("Attempted to set macro: {macro_name} twice")
                    }
                    ctx.macros.insert(
                        macro_name.to_string(),
                        Macro {
                            args,
                            content: macro_body.trim_start_matches(':').trim().to_string(),
                        },
                    );
                } else if l.starts_with("static") {
                    let tokens = l
                        .split(' ')
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>();

                    if tokens.len() != 5 {
                        panic!("Expected @static to be: `@static {{byte|word|dble}} {{name}} = {{value}}`. Got `{l}`")
                    }

                    let name = match tokens.get(2) {
                        Some(&v) => v,
                        _ => panic!("Invalid static definition syntax"),
                    };

                    match tokens.get(3) {
                        Some(&"=") => {}
                        _ => panic!("Invalid static definition syntax"),
                    };

                    let val = match tokens.get(4) {
                        Some(&v) => match try_read_num(v) {
                            Err(_) => panic!(
                                "Failed to set static value {v} for {name}. Bad number syntax"
                            ),

                            Ok(v) => v,
                        },
                        _ => panic!("Invalid static definition syntax"),
                    };

                    let ty = match tokens.get(1) {
                        Some(&"byte") => SymbolType::StaticByte(val as u8),
                        Some(&"word") => SymbolType::StaticWord(val as u16),
                        Some(&"dble") => SymbolType::StaticDouble(val as u32),
                        _ => panic!("Invalid static definition syntax"),
                    };

                    if ctx.symbols.contains_key(name) {
                        panic!("Attempted to set symbol: {name} twice")
                    }
                    ctx.symbols.insert(name.to_string(), ty);
                } else if l.starts_with("data") {
                    let tokens = l
                        .split(' ')
                        .map(|t| t.trim())
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>();

                    if tokens.len() != 3 {
                        panic!("Expected @data to be: `@data {{byte|word|dble}} {{name}}`")
                    }

                    let name = match tokens.get(2) {
                        Some(&v) => v,
                        _ => panic!("Invalid data definition syntax"),
                    };

                    let ty = match tokens.get(1) {
                        Some(&"byte") => SymbolType::MemByte,
                        Some(&"word") => SymbolType::MemWord,
                        Some(&"dble") => SymbolType::MemDouble,
                        _ => panic!("Invalid data definition syntax"),
                    };

                    if ctx.symbols.contains_key(name) {
                        panic!("Attempted to set symbol: {name} twice")
                    }
                    ctx.symbols.insert(name.to_string(), ty);
                } else if let Some(p) = l.strip_prefix("use") {
                    let p = p.trim().trim_matches('"');
                    let pat = PathBuf::from(p);

                    if ctx.files_imported.contains(&pat) {
                        panic!("Attempting to circularly import: {p}")
                    }

                    ctx.files_imported.push(pat.clone());
                    let file = match fs::read(pat) {
                        Ok(f) => String::from_utf8(f).unwrap(),
                        Err(_) => panic!("Failed to read {p}"),
                    };

                    let chunk = scan_with_ctx(&file, ctx);
                    out.push_str(&chunk);
                }
            }
            None => {
                if line.trim().ends_with(':') {
                    let name = line.trim().trim_end_matches(':').trim();
                    if name.is_empty() {
                        panic!("Expected label name")
                    }
                    if ctx.symbols.contains_key(name) {
                        panic!("Attempted to set label: {name} twice")
                    }

                    let mut section = String::new();
                    let lines = source.lines().collect::<Vec<_>>();
                    for j in i..lines.len() {
                        if j == i {
                            skip += 1;
                            continue;
                        }
                        let current = lines.get(j).unwrap();
                        if !current.starts_with(' ') {
                            break;
                        }
                        skip += 1;
                        let mut current = current.trim();

                        if let Some(comm_ind) = current.find('#') {
                            current = &current[0..comm_ind].trim();
                        };

                        if current.is_empty() {
                            continue;
                        }
                        section.push('\n');
                        section.push_str(current);
                    }
                    let section = section.trim().to_string();

                    if section.is_empty() {
                        panic!("Found empty section: {name}");
                    }

                    ctx.symbols.insert(name.to_string(), SymbolType::Label);
                    ctx.sections.push((name.to_string(), section));
                } else {
                    out.push_str(&format!("{}\n", line))
                }
            }
        }
    }
    out
}

#[derive(Debug, Default)]
pub struct Ctx {
    files_imported: Vec<PathBuf>,
    pub symbols: HashMap<String, SymbolType>,
    pub macros: HashMap<String, Macro>,
    pub sections: Vec<(String, String)>,
}

impl Ctx {
    pub fn resolve_macros(&mut self) {
        for (_, section) in self.sections.iter_mut() {
            *section = resolve_macros::resolve_macros(section, &self.macros);
        }
    }
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
    StaticByte(u8),
    StaticWord(u16),
    StaticDouble(u32),
    MemByte,
    MemWord,
    MemDouble,
}

#[derive(Debug)]
pub struct Macro {
    pub args: Vec<String>,
    pub content: String,
}
