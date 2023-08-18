use core::panic;
use std::collections::HashMap;

use regex::{Captures, Regex};

pub fn source(file: &str) -> (String, HashMap<String, Vec<Macro>>) {
    let mut macros: HashMap<String, Vec<Macro>> = HashMap::new();

    let file = MACRO_DEF
        .replace_all(file, |caps: &Captures| {
            let raw = caps.get(0).unwrap().as_str();
            let mac = Macro::from(raw);

            if !macros.contains_key(&mac.id) {
                macros.insert(mac.id.clone(), vec![]);
            }

            let mac_list = macros.get_mut(&mac.id).unwrap();

            mac_list.push(mac);

            ""
        })
        .to_string();

    (file, macros)
}

pub struct Macro {
    pub id: String,
    pub args: Vec<MacroArg>,
    pub body: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MacroArg {
    Register(String),
    Immediate(String),
    Index(String),
}

impl From<&str> for MacroArg {
    fn from(value: &str) -> Self {
        let value = value.trim();

        if value.len() < 2 {
            panic!("Arg too short");
        }

        if value.chars().nth(0).unwrap() == '%' {
            if value.chars().nth(1).unwrap() == 'i' {
                return Self::Immediate(value.to_string());
            }
            if value.chars().nth(1).unwrap() == 'r' {
                return Self::Register(value.to_string());
            }
            if value.starts_with("%[") && value.ends_with("]") {
                return Self::Index(value.to_string());
            }
            panic!("Macro arguments bust be either 'i'mmediate, index, or 'r'egister");
        }
        panic!("Macro arguments must start with %: {value:#?}",);
    }
}

impl ToString for MacroArg {
    fn to_string(&self) -> String {
        match self {
            MacroArg::Immediate(n) => n.to_string(),
            MacroArg::Register(r) => r.to_string(),
            MacroArg::Index(i) => i.to_string(),
        }
    }
}

lazy_static! {
    static ref MACRO_DEF: Regex =
        Regex::new(r"(?s)@macro\s*(\w+)\s*([%\w+\s\[\]]*):[\s\n]+(.*?)\n\n").unwrap();
}

impl From<&str> for Macro {
    fn from(value: &str) -> Self {
        let caps = MACRO_DEF.captures(value).expect("Invalid macro syntax");
        let id = caps.get(1).unwrap().as_str().to_string();
        let args = caps
            .get(2)
            .map(|a| {
                a.as_str()
                    .split(' ')
                    .filter(|i| !i.trim().is_empty())
                    .map(MacroArg::from)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let body = caps.get(3).unwrap().as_str().to_string();
        Self { id, args, body }
    }
}
