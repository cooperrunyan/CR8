use std::collections::HashMap;

use super::Compiler;

impl Compiler {
    pub(crate) fn debug(&self) {
        if self.debug.files {
            self.debug_files()
        }
        if self.debug.macros {
            self.debug_macros()
        }
        if self.debug.labels {
            self.debug_labels()
        }
        if self.debug.bin {
            self.debug_bin()
        }
    }

    fn debug_files(&self) {
        println!("\n===== Files Used: =====");
        for file in self.files.iter() {
            println!("  - {file}");
        }
        println!("=======================\n");
    }

    fn debug_macros(&self) {
        println!("\n===== Macros Declared: =====");
        for (name, mac) in self.macros.iter() {
            println!("  - {name}: {:?}", mac.args);
        }
        println!("============================\n");
    }

    fn debug_labels(&self) {
        println!("\n===== Labels: =====");
        for (name, location) in self.labels.iter() {
            println!("  - {name}: {:?}", location);
        }
        println!("===================\n");
    }

    fn debug_bin(&self) {
        let mut label_reverse_lookup: HashMap<usize, &str> = HashMap::new();

        for (name, location) in self.labels.iter() {
            label_reverse_lookup.insert(*location, &name);
        }

        println!("\n===== Binary: =====");
        for (location, byte) in self.bin.iter().enumerate() {
            if let Some(label) = label_reverse_lookup.get(&location) {
                println!("{location}:  {byte}  -  {:?}", label);
            } else {
                println!("{location}:  {byte}")
            }
        }
        println!("===================\n");
    }
}
