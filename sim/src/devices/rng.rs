use log::warn;
use rand::random;

#[derive(Debug, Default)]
pub struct Rng;

impl Rng {
    pub fn send(&self) -> u8 {
        random::<u8>()
    }

    pub fn receive(&self) {
        warn!("Rng cannot recieve data")
    }
}
