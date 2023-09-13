use asm::reg::Register;

use super::{Joinable, Splittable, CR8};

impl CR8 {
    pub fn hl(&self) -> (u8, u8) {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        (l, h)
    }

    pub fn sp(&self) -> (u8, u8) {
        (self.spl, self.sph)
    }

    pub fn pc(&self) -> (u8, u8) {
        (self.pcl, self.pch)
    }

    pub fn set_sp(&mut self, (l, h): (u8, u8)) {
        self.spl = l;
        self.sph = h;
    }

    pub fn inc_pc(&mut self) {
        let pc = self.pc().join();
        let (pcl, pch) = (pc + 1).split();
        self.pcl = pcl;
        self.pch = pch;
    }
}
