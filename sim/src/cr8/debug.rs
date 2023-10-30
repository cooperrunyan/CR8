use crate::devices::DeviceSnapshot;

use super::*;
use log::info;

macro_rules! byte {
    ($name:expr, $val:expr) => {
        format!("{}:  {:#04x} | {:#3} | {:08b}", $name, $val, $val, $val)
    };
}

macro_rules! addr {
    ($name:expr, $val:expr, $pt:expr) => {
        format!(
            "{}: {{ {:#06x} | {:#5} }}  =>  {:#3} | {:#04x}",
            $name, $val, $val, $pt, $pt
        )
    };
}

impl CR8 {
    pub fn debug(&self, mem: &Mem, dev: DeviceSnapshot) {
        let snapshot = String::new();

        #[cfg(feature = "keyboard")]
        let snapshot = {
            let mut snapshot = snapshot;
            snapshot.push_str(&format!("\n    - keyboard: {:#010b}", dev.keyboard));
            snapshot
        };

        #[cfg(feature = "rng")]
        let snapshot = {
            let mut snapshot = snapshot;
            snapshot.push_str(&format!("\n    - rng: {:#010b}", dev.rng));
            snapshot
        };

        info!(
            "\n{}",
            format!(
                r#"
================== State ==================

  {}
  {}
  {}
  {}
  {}
  {}
  {}
  {}

  {}
  {}
  {}


  Memory banks:
{:?}

  Devices:
{}

===========================================
"#,
                byte!("A", self.reg[Register::A as usize]),
                byte!("B", self.reg[Register::B as usize]),
                byte!("C", self.reg[Register::C as usize]),
                byte!("D", self.reg[Register::D as usize]),
                byte!("F", self.reg[Register::F as usize]),
                byte!("X", self.reg[Register::X as usize]),
                byte!("Y", self.reg[Register::Y as usize]),
                byte!("Z", self.reg[Register::Z as usize]),
                addr!("XY", self.xy(), mem.get(self.xy()).unwrap_or_default()),
                addr!("PC", self.pc, mem.get(self.pc).unwrap_or_default()),
                addr!("SP", self.sp, mem.get(self.sp).unwrap_or_default()),
                mem.banks,
                snapshot
            )
        );
    }
}
