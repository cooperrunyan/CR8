use log::{debug, trace};
use std::collections::HashMap;

use super::Compiler;

impl Compiler {
    pub(crate) fn debug(&self) {
        self.debug_labels();
        self.debug_files();
        self.debug_statics();
        self.debug_macros();
        self.debug_bin();
    }

    fn debug_files(&self) {
        debug!("===== Files Used: =====");
        for file in self.files.iter() {
            debug!("  - {file}");
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

    fn debug_macros(&self) {
        trace!("===== Macros Declared: =====");
        for (name, mac) in self.macros.iter() {
            trace!("  - {name}: {:?}", mac.args);
        }
        trace!("");
    }

    fn debug_labels(&self) {
        debug!("===== Labels: =====");
        let mut sorted: Vec<_> = self.labels.iter().collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

        for (name, location) in sorted {
            if name.starts_with("#marker ") {
                continue;
            }
            debug!("  - {name}: {:?}", location);
        }
        debug!("");
    }

    fn debug_bin(&self) {
        let mut label_reverse_lookup: HashMap<usize, &str> = HashMap::new();

        for (name, location) in self.labels.iter() {
            label_reverse_lookup.insert(*location, &name);
        }

        trace!("===== Binary: =====");
        for (location, byte) in self.bin.iter().enumerate() {
            if let Some(label) = label_reverse_lookup.get(&location) {
                if label.starts_with("#marker ") {
                    trace!("  {location:04x}:  {byte:02x}  --  {label}");
                } else {
                    trace!("");
                    trace!("{}:", label);
                    trace!("  {location:04x}:  {byte:02x}");
                }
            } else {
                trace!("  {location:04x}:  {byte:02x}");
            }
        }
        trace!("");
    }
}
