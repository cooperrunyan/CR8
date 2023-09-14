use super::*;

impl CR8 {
    pub fn debug(&self) {
        println!("\n\n========== State ==========");
        println!("A: {}", self.reg[Register::A as usize]);
        println!("B: {}", self.reg[Register::B as usize]);
        println!("C: {}", self.reg[Register::C as usize]);
        println!("D: {}", self.reg[Register::D as usize]);
        println!("Z: {}", self.reg[Register::Z as usize]);
        let f = self.reg[Register::F as usize];

        println!("F: {f}");
        println!("  EMPTY BF CF EF LF");
        print!("  ");
        for i in (0..=7).rev() {
            if (f >> i) & 1 == 1 {
                print!("1");
            } else {
                print!("0");
            }
            if 7 - i >= 3 {
                print!("  ");
            }
        }
        println!();
        println!();
        println!("HL: {}", self.hl());
        println!("[HL]: {:?}", self.mem.get(self.hl()));
        println!("SP: {}", self.sp - STACK);
        println!("[SP]: {:?}", self.mem.get(self.sp));

        println!();
        println!("Memory banks:");
        println!("{:?}", self.mem.banks);

        println!();

        println!("===========================\n");
    }
}
