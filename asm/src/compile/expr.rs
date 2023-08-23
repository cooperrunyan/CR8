use std::collections::HashMap;

use cfg::mem;
use regex::{Captures, Regex};

use super::scan::SymbolType;

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"\[([^\]]*)\)\]").unwrap();
    static ref SPACE_RE: Regex = Regex::new(r"\s*").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"\w[\w\d]*").unwrap();
    static ref DOT_RE: Regex = Regex::new(r"\D(\.)\D").unwrap();
    static ref EXPR_HEX_RE: Regex = Regex::new(r"0x([0-9a-f]+)").unwrap();
    static ref EXPR_BIN_RE: Regex = Regex::new(r"0b([01]+)").unwrap();
    static ref EXPR_GRP_RE: Regex = Regex::new(r"\((?:[^()]+)*\)").unwrap();
    static ref EXPR_MUL_RE: Regex = Regex::new(r"\d+(\s*\*\s*\d+)+").unwrap();
    static ref EXPR_DIV_RE: Regex = Regex::new(r"\d+(\s*\/\s*\d+)+").unwrap();
    static ref EXPR_ADD_RE: Regex = Regex::new(r"\d+(\s*\+\s*\d+)+").unwrap();
    static ref EXPR_SUB_RE: Regex = Regex::new(r"\d+(\s*\-\s*\d+)+").unwrap();
    static ref EXPR_AND_RE: Regex = Regex::new(r"\d+(\s*\&\s*\d+)+").unwrap();
    static ref EXPR_OR_RE: Regex = Regex::new(r"\d+(\s*\|\s*\d+)+").unwrap();
    static ref EXPR_LSH_RE: Regex = Regex::new(r"\d+\s*<<\s*\d+").unwrap();
    static ref EXPR_RSH_RE: Regex = Regex::new(r"\d+\s*>>\s*\d+").unwrap();
}

pub fn parse(
    expr: &str,
    pc: usize,
    symbols: &HashMap<String, SymbolType>,
    sections: &HashMap<String, usize>,
    variables: &HashMap<String, usize>,
) -> String {
    let expr = SYM_RE
        .replace_all(&expr, |caps: &Captures| {
            let symbol = caps.get(0).unwrap().as_str().trim();

            if let Some(val) = symbols.get(symbol) {
                match val {
                    SymbolType::Label => match sections.get(symbol) {
                        None => panic!("Undefined section: {symbol}"),
                        Some(i) => i.to_owned().to_string(),
                    },
                    SymbolType::StaticByte(value) => value.to_owned().to_string(),
                    SymbolType::StaticWord(value) => value.to_owned().to_string(),
                    SymbolType::StaticDouble(value) => value.to_owned().to_string(),
                    SymbolType::MemByte => (mem::GP_RAM
                        + variables
                            .get(symbol)
                            .expect(&format!("Undefined variable: {symbol}"))
                            .to_owned() as u16)
                        .to_string(),
                    SymbolType::MemWord => (mem::GP_RAM
                        + variables
                            .get(symbol)
                            .expect(&format!("Undefined variable: {symbol}"))
                            .to_owned() as u16)
                        .to_string(),
                    SymbolType::MemDouble => (mem::GP_RAM
                        + variables
                            .get(symbol)
                            .expect(&format!("Undefined variable: {symbol}"))
                            .to_owned() as u16)
                        .to_string(),
                }
            } else {
                symbol.to_string()
            }
        })
        .to_string();

    let expr = DOT_RE.replace(&expr, pc.to_string()).to_string();

    let expr = EXPR_HEX_RE
        .replace_all(&expr, |caps: &Captures| {
            let hex_str = caps.get(1).unwrap().as_str();
            u64::from_str_radix(hex_str, 16).unwrap().to_string()
        })
        .to_string();

    let expr = EXPR_BIN_RE
        .replace_all(&expr, |caps: &Captures| {
            let bin_str = caps.get(1).unwrap().as_str();
            u64::from_str_radix(bin_str, 2).unwrap().to_string()
        })
        .to_string();

    evaluate_grp(&expr.trim_end_matches("]").trim_start_matches("["))
}

fn evaluate_grp(expr: &str) -> String {
    let mut expr = expr.to_string();
    let t_expr = expr.clone();
    let groups = EXPR_GRP_RE.captures(&t_expr);
    if let Some(g) = groups {
        for group in g.iter() {
            let group = match group {
                None => continue,
                Some(a) => a,
            };
            let end = group.as_str().len() - 1;
            let res = evaluate_grp(&group.as_str()[1..end]);
            expr = expr.replace(group.as_str(), &res);
        }
    };

    expr = EXPR_MUL_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split('*').filter(|i| i != &"*").collect::<Vec<_>>();

            let mut accum: u64 = 1;

            for numstr in nums {
                accum *= numstr.trim().parse::<u64>().expect("Invalid number");
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_DIV_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split('/').filter(|i| i != &"/").collect::<Vec<_>>();

            let mut accum: u64 = 1;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum /= numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_ADD_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split('+').filter(|i| i != &"+").collect::<Vec<_>>();

            let mut accum: u64 = 0;

            for numstr in nums {
                accum += numstr.trim().parse::<u64>().expect("Invalid number");
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_SUB_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split('-').filter(|i| i != &"-").collect::<Vec<_>>();

            let mut accum: u64 = 1;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum -= numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_LSH_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split("<<").filter(|i| i != &"<<").collect::<Vec<_>>();

            let mut accum: u64 = 0;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum = accum << numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_RSH_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split(">>").filter(|i| i != &">>").collect::<Vec<_>>();

            let mut accum: u64 = 0;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum = accum >> numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_AND_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split("&").filter(|i| i != &"&").collect::<Vec<_>>();

            let mut accum: u64 = 0;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum = accum & numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr = EXPR_OR_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split("|").filter(|i| i != &"|").collect::<Vec<_>>();

            let mut accum: u64 = 0;

            for (i, numstr) in nums.iter().enumerate() {
                if i == 0 {
                    accum = numstr.trim().parse::<u64>().expect("Invalid number")
                } else {
                    accum = accum | numstr.trim().parse::<u64>().expect("Invalid number");
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr
}
