use std::collections::HashMap;

use regex::{Captures, Regex};

use super::Size;

lazy_static! {
    static ref REF_DEFINE_RE: Regex = Regex::new(r"@store +(byte|word|dble) +(\S+)").unwrap();
}

pub fn get_store_stmts(file: &str) -> (String, HashMap<String, usize>) {
    let mut stores: Vec<(String, Size)> = vec![];

    let file = REF_DEFINE_RE
        .replace_all(file, |cap: &Captures| {
            let id = cap.get(2).unwrap().as_str().to_string();
            let ty = Size::from(cap.get(1).unwrap().as_str());

            for (i, _) in stores.iter() {
                if i == &id {
                    panic!("Attempting to set {id} twice")
                }
            }

            stores.push((id.clone(), ty));

            if ty == Size::Word || ty == Size::Double {
                stores.push(("".to_string(), Size::Byte));
            }

            if ty == Size::Double {
                stores.push(("".to_string(), Size::Byte));
                stores.push(("".to_string(), Size::Byte));
            }

            ""
        })
        .to_string();

    let mut map = HashMap::new();

    for (i, (name, _)) in stores.iter().enumerate() {
        if name.is_empty() {
            continue;
        }

        map.insert(name.to_string(), i);
    }

    (file, map)
}

impl From<&str> for Size {
    fn from(value: &str) -> Self {
        match value {
            "byte" => Self::Byte,
            "word" => Self::Word,
            "dble" => Self::Double,
            _ => panic!("Invalid @store type"),
        }
    }
}
