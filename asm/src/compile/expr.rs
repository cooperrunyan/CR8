use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::compile::parse_num;

lazy_static! {
    pub static ref EXPR_RE: Regex = Regex::new(r"`(?<eval>[^`]*)`").unwrap();
    pub static ref SPACE_RE: Regex = Regex::new(r"\s*").unwrap();
    pub static ref EXPR_GRP_RE: Regex = Regex::new(r"\((?:[^()]+)*\)").unwrap();
    pub static ref EXPR_MUL_RE: Regex = Regex::new(r"\d+(\s*\*\s*\d+)+").unwrap();
    pub static ref EXPR_DIV_RE: Regex = Regex::new(r"\d+(\s*\/\s*\d+)+").unwrap();
    pub static ref EXPR_ADD_RE: Regex = Regex::new(r"\d+(\s*\+\s*\d+)+").unwrap();
    pub static ref EXPR_SUB_RE: Regex = Regex::new(r"\d+(\s*\-\s*\d+)+").unwrap();
    pub static ref REF_USE_RE: Regex = Regex::new(r"&(\S+)").unwrap();
}

pub fn expr(expr: &str, refs: &HashMap<String, u64>) -> u64 {
    let expr = SPACE_RE.replace_all(expr, |_: &Captures| "");
    let expr = REF_USE_RE.replace_all(&expr, |caps: &Captures| {
        let ref_name = caps
            .get(1)
            .expect(&format!("Expected a reference name {expr}",))
            .as_str();
        let ref_val = refs
            .get(ref_name)
            .expect(&format!("Reference: {ref_name} is not defined"));
        format!("{ref_val}")
    });

    let r = expr_grp(&expr);

    r.parse::<u64>()
        .expect(&format!("Evaluation result failed: {expr}"))
}

fn expr_grp(expr: &str) -> String {
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
            let res = expr_grp(&group.as_str()[1..end]);
            expr = expr.replace(group.as_str(), &res);
        }
    };

    expr = EXPR_MUL_RE
        .replace(&expr, |caps: &Captures| {
            let str = caps.iter().next().unwrap().unwrap().as_str();
            let nums = str.split('*').filter(|i| i != &"*").collect::<Vec<_>>();

            let mut accum: u64 = 1;

            for numstr in nums {
                accum *= parse_num::<u64>(numstr);
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
                    accum = parse_num(numstr)
                } else {
                    accum /= parse_num::<u64>(numstr);
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
                accum += parse_num::<u64>(numstr);
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
                    accum = parse_num(numstr)
                } else {
                    accum -= parse_num::<u64>(numstr);
                }
            }

            format!("{accum}")
        })
        .to_string();

    expr
}
