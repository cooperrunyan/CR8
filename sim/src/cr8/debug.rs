use crate::devices::DeviceSnapshot;

use super::*;
use log::info;

impl CR8 {
    pub fn debug(&self, mem: &Mem, dev: DeviceSnapshot) {
        let mut snapshot = String::new();

        snapshot.push_str(&format!("    - sysctrl: {:#010b}", dev.sysctrl));

        #[cfg(feature = "keyboard")]
        snapshot.push_str(&format!("\n    - keyboard: {:#010b}", dev.keyboard));

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
                byte!("Z", self.reg[Register::Z as usize]),
                byte!("F", self.reg[Register::F as usize]),
                byte!("L", self.reg[Register::L as usize]),
                byte!("H", self.reg[Register::H as usize]),
                addr!("HL", self.hl(), mem.get(self.hl()).unwrap_or_default()),
                addr!("PC", self.pc, mem.get(self.pc).unwrap_or_default()),
                addr!("SP", self.sp, mem.get(self.sp).unwrap_or_default()),
                mem.banks,
                snapshot
            )
        );
    }
}
