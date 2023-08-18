use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::compiler::directive::def::Definition;

lazy_static! {
    pub static ref EXPR_RE: Regex = Regex::new(r"\[([^\]]*)\)\]").unwrap();
    static ref SPACE_RE: Regex = Regex::new(r"\s*").unwrap();
    static ref EXPR_GRP_RE: Regex = Regex::new(r"\((?:[^()]+)*\)").unwrap();
    static ref EXPR_MUL_RE: Regex = Regex::new(r"\d+(\s*\*\s*\d+)+").unwrap();
    static ref EXPR_DIV_RE: Regex = Regex::new(r"\d+(\s*\/\s*\d+)+").unwrap();
    static ref EXPR_ADD_RE: Regex = Regex::new(r"\d+(\s*\+\s*\d+)+").unwrap();
    static ref EXPR_SUB_RE: Regex = Regex::new(r"\d+(\s*\-\s*\d+)+").unwrap();
    static ref DEF_USE_RE: Regex = Regex::new(r"&([\w_\d]+)").unwrap();
}

pub fn parse(file: &str, defs: &HashMap<String, Definition>) -> String {
    EXPR_RE
        .replace_all(file, |caps: &Captures| {
            evaluate(caps.get(0).unwrap().as_str(), defs)
        })
        .to_string()
}

pub fn evaluate(expr: &str, defs: &HashMap<String, Definition>) -> String {
    let expr = DEF_USE_RE.replace_all(&expr, |caps: &Captures| {
        let def_name = caps
            .get(1)
            .expect(&format!("Expected a reference name {expr}",))
            .as_str()
            .trim();

        let def_val = defs
            .get(def_name)
            .expect(&format!("Reference: {def_name} is not defined"));
        format!("{}", def_val.val.raw())
    });

    let r = evaluate_grp(&expr.trim_end_matches("]").trim_start_matches("["));

    format!(
        "${}D",
        r.parse::<u64>()
            .expect(&format!("Evaluation result failed: {expr}, {r}"))
    )
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

    expr
}
