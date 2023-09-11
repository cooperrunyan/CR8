use asm::reg::Register;

use super::{join, split, CR8, PROGRAM_COUNTER, STACK_POINTER};

impl CR8 {
    pub(super) fn hl(&self) -> (u8, u8) {
        let l = self.reg[Register::L as usize];
        let h = self.reg[Register::H as usize];

        (l, h)
    }

    pub(super) fn sp(&self) -> (u8, u8) {
        let spl = self.mem[STACK_POINTER as usize];
        let sph = self.mem[(STACK_POINTER + 1) as usize];

        (spl, sph)
    }

    pub(super) fn pc(&self) -> (u8, u8) {
        let pcl = self.mem[PROGRAM_COUNTER as usize];
        let pch = self.mem[(PROGRAM_COUNTER + 1) as usize];

        (pcl, pch)
    }

    pub(super) fn set_sp(&mut self, (l, h): (u8, u8)) {
        self.mem[STACK_POINTER as usize] = l;
        self.mem[(STACK_POINTER + 1) as usize] = h;
    }

    pub(super) fn inc_pc(&mut self) {
        let pc = join(self.pc());
        let (pcl, pch) = split(pc + 1);
        self.mem[PROGRAM_COUNTER as usize] = pcl;
        self.mem[(PROGRAM_COUNTER + 1) as usize] = pch;
    }
}
