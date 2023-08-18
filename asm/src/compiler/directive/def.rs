use std::collections::HashMap;

use cfg::mem::{Size, SizedValue};
use regex::{Captures, Regex};

use crate::compiler::parse::expr::{self, EXPR_RE};

lazy_static! {
    pub static ref DEFINE_RE: Regex =
        Regex::new(r"@def +(byte|word|dble) +(\S+)\s*=\s*(.+)").unwrap();
}

pub fn collect(file: &str) -> (String, HashMap<String, Definition>) {
    let mut defs: HashMap<String, Definition> = HashMap::new();

    let file = DEFINE_RE
        .replace_all(file, |cap: &Captures| {
            let id = cap.get(2).unwrap().as_str().to_string();

            if defs.contains_key(&id) {
                panic!("Attempting to set {id} twice")
            }

            let ty_str = cap.get(1).unwrap().as_str();
            let ty = Size::from(ty_str);

            let mut val_str = cap.get(3).unwrap().as_str().to_string();

            if EXPR_RE.is_match(&val_str) {
                val_str = expr::evaluate(&val_str, &defs);
            }

            let val = ty.val(&val_str);

            defs.insert(id.clone(), Definition { id, val });

            ""
        })
        .to_string();

    (file, defs)
}

pub struct Definition {
    pub id: String,
    pub val: SizedValue,
}
