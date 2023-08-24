#[macro_use]
extern crate lazy_static;

mod compile;

use std::collections::HashMap;

const PADDING: u64 = 0;

pub fn compile(source: &str) -> Vec<u8> {
    let mut ctx = compile::scan(source);

    ctx.resolve_macros();

    let mut variables: HashMap<String, usize> = HashMap::new();
    let mut acc = 0;

    for (sym, ty) in ctx.symbols.iter() {
        match ty {
            compile::SymbolType::MemByte => {
                variables.insert(sym.to_string(), acc);
                acc += 1;
            }
            compile::SymbolType::MemWord => {
                variables.insert(sym.to_string(), acc);
                acc += 2;
            }
            compile::SymbolType::MemDouble => {
                variables.insert(sym.to_string(), acc);
                acc += 4;
            }
            _ => {}
        }
    }

    let mut section_sizes: HashMap<String, usize> = HashMap::new();

    for (name, section) in ctx.sections.iter() {
        let mut size: usize = 0;
        for line in section.lines() {
            let (inst, args) = line.trim().split_once(' ').unwrap_or((line.trim(), ""));

            if inst.is_empty() {
                continue;
            }

            size += 1;

            let args = args.split(',').map(|a| a.trim()).filter(|a| !a.is_empty());

            for _ in args {
                size += 1;
            }
        }
        section_sizes.insert(name.to_string(), size);
    }

    let global_size = section_sizes.remove("global").unwrap();
    let global_section = ctx.sections.remove("global").unwrap();

    let mut section_index_map: HashMap<String, usize> = HashMap::new();

    section_index_map.insert("global".to_string(), 0);

    let mut acc = global_size;

    for (name, size) in section_sizes.iter() {
        acc += PADDING as usize;
        section_index_map.insert(name.to_string(), acc.to_owned());
        acc += size
    }

    let mut bin: Vec<u8> = vec![];

    bin.append(&mut compile::section(
        &global_section,
        section_index_map.get("global").unwrap(),
        &section_index_map,
        &variables,
        &ctx.symbols,
    ));

    for (name, section) in ctx.sections.iter() {
        if name == "global" {
            continue;
        }

        let mut sect = compile::section(
            section,
            section_index_map.get(name).unwrap(),
            &section_index_map,
            &variables,
            &ctx.symbols,
        );

        bin.append(&mut sect);
    }

    bin
}
