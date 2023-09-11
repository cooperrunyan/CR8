use super::*;

impl CR8 {
    pub fn debug(&self) {
        println!("\n\n===== State: =====");
        println!("A: {}", self.reg[Register::A as usize]);
        println!("B: {}", self.reg[Register::B as usize]);
        println!("C: {}", self.reg[Register::C as usize]);
        println!("D: {}", self.reg[Register::D as usize]);
        println!("Z: {}", self.reg[Register::Z as usize]);
        println!("HL: {}", join(self.hl()));
        println!("[HL]: {}", self.memory.get(self.mb, join(self.hl())));
        println!("SP: {}", join(self.sp()) - STACK);
        println!("[SP]: {}", self.memory.get(self.mb, join(self.sp())));
        println!();
        println!("Devices:");

        for (port, dev) in self.dev.iter() {
            println!("  {port}: {}", dev.inspect());
        }

        println!();
        println!("Memory banks:");

        for (bank, _) in self.memory.banks.iter().enumerate() {
            println!("  {}", bank + 1);
        }

        println!();
        let f = self.reg[Register::F as usize];
        let lf = f & 1;
        let ef = (f >> 1) & 1;
        let cf = (f >> 2) & 1;
        let bf = (f >> 3) & 1;

        println!();
        println!("LF: {}", lf == 1);
        println!("EF: {}", ef == 1);
        println!("CF: {}", cf == 1);
        println!("BF: {}", bf == 1);
        println!("==================\n");
    }
}
