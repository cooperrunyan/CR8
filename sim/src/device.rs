pub trait Device {
    fn receive(&mut self, byte: u8);
    fn send(&mut self) -> u8;
    fn inspect(&self) -> u8;
}

#[derive(Default, Debug)]
pub struct Control {
    pub state: u8,
}

impl Device for Control {
    fn receive(&mut self, byte: u8) {
        match byte {
            0x00 => println!("Control recieved NOP message"),
            0x01 => {
                self.state |= 0b00000001;
            }
            _ => {}
        }
    }

    fn send(&mut self) -> u8 {
        self.state
    }

    fn inspect(&self) -> u8 {
        self.state
    }
}
