use std::num::ParseIntError;

use regex::{Captures, Regex};

use crate::compiler::Compiler;

lazy_static! {
    static ref EXPR_RE: Regex = Regex::new(r"\[([^\]]*)\]").unwrap();
    static ref SPACE_RE: Regex = Regex::new(r"\s*").unwrap();
    static ref SYM_RE: Regex = Regex::new(r"\&?[\w_\.][\.\w\d_]*").unwrap();
    static ref DOT_RE: Regex = Regex::new(r"(\$)").unwrap();
    static ref EXPR_HEX_RE: Regex = Regex::new(r"0x(?i:([0-9a-f]+))").unwrap();
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

pub fn parse(expr: &str, compiler: &Compiler) -> Result<i128, String> {
    let expr = SYM_RE.replace_all(expr, |caps: &Captures| {
        let symbol = caps.get(0).unwrap().as_str().trim();

        if let Some(stat) = compiler.ast.statics.get(symbol) {
            (stat.to_owned() as i128).to_string()
        } else if let Some(label) = compiler.labels.get(symbol) {
            (label.to_owned() as i128).to_string()
        } else if let Some(label) = compiler
            .labels
            .get(&format!("{}{}", compiler.last_label, symbol))
        {
            (label.to_owned() as i128).to_string()
        } else if let Some(ram_loc) = compiler.ast.ram_locations.get(symbol) {
            ((*ram_loc as i128) + compiler.ast.ram_origin as i128).to_string()
        } else {
            symbol.to_string()
        }
    });

    let expr = DOT_RE.replace(&expr, compiler.bin.len().to_string());

    let expr = EXPR_HEX_RE.replace_all(&expr, |caps: &Captures| {
        let hex_str = caps.get(1).unwrap().as_str();
        i128::from_str_radix(hex_str, 16).unwrap().to_string()
    });

    let expr = EXPR_BIN_RE.replace_all(&expr, |caps: &Captures| {
        let bin_str = caps.get(1).unwrap().as_str();
        i128::from_str_radix(bin_str, 2).unwrap().to_string()
    });

    let r = evaluate_grp(expr.to_string().as_str(), compiler);
    dbg!(&r);
    r
}

fn try_read_num(raw: &str) -> Result<i128, ParseIntError> {
    let raw = raw.trim();

    if let Some(b) = raw.strip_prefix("0b") {
        i128::from_str_radix(b, 2)
    } else if let Some(h) = raw.strip_prefix("0x") {
        i128::from_str_radix(h, 16)
    } else {
        raw.parse()
    }
}

fn evaluate_grp(expr: &str, compiler: &Compiler) -> Result<i128, String> {
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
            let res = evaluate_grp(&group.as_str()[1..end], compiler)?;
            expr = expr.replace(group.as_str(), &res.to_string());
        }
    };

    let expr = EXPR_MUL_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 1;
        let str = caps.get(0).unwrap().as_str();

        for numstr in str.split('*').filter(|i| i != &"*") {
            acc *= try_read_num(numstr).expect("Invalid number");
        }

        acc.to_string()
    });

    let expr = EXPR_DIV_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 1;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split('/').filter(|i| i != &"/").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc /= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    let expr = EXPR_ADD_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 0;
        let str = caps.get(0).unwrap().as_str();

        for numstr in str.split('+').filter(|i| i != &"+") {
            acc += try_read_num(numstr).expect("Invalid number");
        }

        acc.to_string()
    });

    let expr = EXPR_SUB_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 1;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split('-').filter(|i| i != &"-").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc -= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    let expr = EXPR_LSH_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 0;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split("<<").filter(|i| i != &"<<").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc <<= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    let expr = EXPR_RSH_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 0;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split(">>").filter(|i| i != &">>").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc >>= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    let expr = EXPR_AND_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 0;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split('&').filter(|i| i != &"&").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc &= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    let expr = EXPR_OR_RE.replace(&expr, |caps: &Captures| {
        let mut acc: i128 = 0;
        let str = caps.get(0).unwrap().as_str();

        for (i, numstr) in str.split('|').filter(|i| i != &"|").enumerate() {
            if i == 0 {
                acc = try_read_num(numstr).expect("Invalid number")
            } else {
                acc |= try_read_num(numstr).expect("Invalid number");
            }
        }

        acc.to_string()
    });

    expr.to_string()
        .parse::<i128>()
        .map_err(|_| format!("Bad int: {expr:#?}"))
}
