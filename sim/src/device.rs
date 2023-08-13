use crate::cr8::CR8;

pub struct Device {
    pub id: u8,
    pub name: String,
    pub recieve: Box<dyn Fn(&Device, &CR8, u8)>,
    pub send: Box<dyn Fn(&Device, &CR8) -> u8>,
    pub drop: Box<dyn Fn(&Device, &CR8)>,
}
