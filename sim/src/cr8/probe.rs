use asm::reg::Register;

use super::{join, split, CR8};

impl CR8 {
    pub(super) fn hl(&self) -> (u8, u8) {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        (l, h)
    }

    pub(super) fn sp(&self) -> (u8, u8) {
        (self.spl, self.sph)
    }

    pub(super) fn pc(&self) -> (u8, u8) {
        (self.pcl, self.pch)
    }

    pub(super) fn set_sp(&mut self, (l, h): (u8, u8)) {
        self.spl = l;
        self.sph = h;
    }

    pub(super) fn inc_pc(&mut self) {
        let pc = join(self.pc());
        let (pcl, pch) = split(pc + 1);
        self.pcl = pcl;
        self.pch = pch;
    }
}
