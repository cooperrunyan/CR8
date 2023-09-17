use std::fmt::Debug;

use log::info;

#[derive(Debug, Default, Clone, Copy)]
pub struct Keyboard(pub u8);

impl Keyboard {
    pub fn get(&self, key: impl Into<Key>) -> bool {
        let key: Key = key.into();
        let key = key as u8;
        let mask = 1 << key;
        self.0 & mask == mask
    }

    pub fn set(&mut self, key: impl Into<Key>, pressed: bool) {
        let key: Key = key.into();
        info!("{key:?} {pressed}");
        let key = key as u8;
        let mask = 1 << key;
        self.0 &= !mask;
        if pressed {
            self.0 |= mask;
        }
    }

    pub fn flush(&mut self) -> u8 {
        let state = self.0;
        self.0 = 0;
        state
    }
}

// 3 bits
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    R,
    Plus,
    Minus,
}
