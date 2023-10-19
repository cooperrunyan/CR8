use log::warn;
use rand::random;

/// Allows the [crate::cr8::CR8] to request a random byte.
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
