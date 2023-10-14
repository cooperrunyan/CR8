use log::{debug, info, log_enabled, Level};
use path_clean::clean;
use std::collections::HashMap;
use std::env;

use super::Compiler;

impl Compiler {
    pub(crate) fn debug(&self) {
        self.debug_labels();
        self.debug_files();
        self.debug_statics();
        self.debug_vars();

        if log_enabled!(Level::Debug) {
            self.debug_macros();
            self.debug_bin();
        }
    }

    fn debug_files(&self) {
        info!("===== Files Used: =====");
        let mut pwd = env::current_dir().unwrap().display().to_string();
        pwd.push('/');
        for file in self.files.iter() {
            info!("  - {}", clean(file.as_path()).display());
        }
        info!("");
    }

    fn debug_statics(&self) {
        info!("======== Statics: ========");
        for (k, v) in self.statics.iter() {
            info!("  - {}: {:#06X}", k, v);
        }
        info!("");
    }

    fn debug_vars(&self) {
        info!("======= Variables: ========");
        for (k, v) in self.ram_locations.iter() {
            info!("  - {}: {:#06X}", k, *v);
        }
        info!("");
    }

    fn debug_macros(&self) {
        debug!("===== Macros Declared: =====");
        for (name, _) in self.macros.iter() {
            debug!("  - {name}");
        }
        debug!("");
    }

    fn debug_labels(&self) {
        info!("===== Labels: =====");
        for (name, location) in self.labels.iter() {
            info!("  - {name}: {:?}", location);
        }
        info!("");
    }

    fn debug_bin(&self) {
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
