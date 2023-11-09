use log::debug;
use path_clean::clean;
use std::collections::HashMap;
use std::env;

use super::Compiler;

impl Compiler {
    pub fn debug(&self) {
        self.debug_bin();
        self.debug_files();
        self.debug_vars();
        self.debug_labels();
        self.debug_statics();
        self.debug_macros();
    }

    fn debug_files(&self) {
        debug!("===== Files Used: =====");
        let mut pwd = env::current_dir().unwrap().display().to_string();
        pwd.push('/');
        for file in self.files.iter() {
            debug!("  - {}", clean(file.as_path()).display());
        }
        debug!("");
    }

    fn debug_statics(&self) {
        debug!("======== Statics: ========");
        for (k, v) in self.statics.iter() {
            debug!("  - {}: {:#06X}", k, v);
        }
        debug!("");
    }

    fn debug_vars(&self) {
        debug!("======= Variables: ========");
        for (k, v) in self.ram_locations.iter() {
            debug!("  - {}: {:#06X}", k, *v);
        }
        debug!("");
    }

    fn debug_macros(&self) {
        debug!("===== Macros Declared: =====");
        for (name, _) in self.macros.iter() {
            debug!("  - {name}");
        }
        debug!("");
    }

    fn debug_labels(&self) {
        debug!("===== Labels: =====");
        for (name, location) in self.labels.iter() {
            debug!("  - {name}: {:?}", location);
        }
        debug!("");
    }

    pub fn debug_bin(&self) {
        let mut label_reverse_lookup: HashMap<usize, &str> = HashMap::new();

        for (name, location) in self.labels.iter() {
            label_reverse_lookup.insert(*location, name);
        }

        debug!("===== Binary: =====");
        for (location, byte) in self.bin.iter().enumerate() {
            if let Some(label) = label_reverse_lookup.get(&location) {
                debug!("");
                debug!("{}:", label);
            }
            debug!("  {location:04x}:  {byte:02x} {byte:3} {byte:08b}");
        }
        debug!("");
    }
}
