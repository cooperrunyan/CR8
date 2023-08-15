use std::collections::HashMap;

use regex::{Captures, Regex};

use super::parse_num;

lazy_static! {
    pub static ref DEFINE_RE: Regex =
        Regex::new(r"@def +(byte|word|dble) +(\S+)\s*=\s*(.+)").unwrap();
}

pub fn read_def_stmts(file: &str) -> (String, HashMap<String, Definition>) {
    let mut defs: HashMap<String, Definition> = HashMap::new();

    let file = DEFINE_RE
        .replace_all(file, |cap: &Captures| {
            let id = cap.get(2).unwrap().as_str().to_string();

            if defs.contains_key(&id) {
                panic!("Attempting to set {id} twice")
            }

            let data = LiteralNumber::from(cap);

            defs.insert(id.clone(), Definition { id, data });

            ""
        })
        .to_string();

    (file, defs)
}

pub struct Definition {
    pub id: String,
    pub data: LiteralNumber,
}

pub enum LiteralNumber {
    Byte(u8),
    Word(u16),
    Double(u32),
}

impl<'c> From<&'c Captures<'c>> for LiteralNumber {
    fn from(value: &'c Captures) -> Self {
        let ty = value.get(1).unwrap().as_str();
        let val = value.get(3).unwrap().as_str();

        match ty {
            "byte" => Self::Byte(parse_num::<u64>(val) as u8),
            "word" => Self::Word(parse_num::<u64>(val) as u16),
            "dble" => Self::Double(parse_num::<u64>(val) as u32),
            _ => panic!("Invalid @def type"),
        }
    }
}
