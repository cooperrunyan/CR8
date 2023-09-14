use super::*;
use log::debug;

impl CR8 {
    pub fn debug(&self) {
        debug!(
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
                addr!("HL", self.hl(), self.mem.get(self.hl()).unwrap_or_default()),
                addr!("PC", self.pc, self.mem.get(self.pc).unwrap_or_default()),
                addr!("SP", self.sp, self.mem.get(self.sp).unwrap_or_default()),
                self.mem.banks
            )
        );
    }
}
