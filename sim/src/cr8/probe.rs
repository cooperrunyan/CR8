use asm::reg::Register;

use super::{Joinable, CR8};

impl CR8 {
    pub fn hl(&self) -> u16 {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        (l, h).join()
    }
}
