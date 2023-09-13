use util_macros::encodable;

encodable! {
    pub enum Register {
        A(0x00, "a"),
        B(0x01, "b"),
        C(0x02, "c"),
        D(0x03, "d"),
        Z(0x04, "z"),
        L(0x05, "l"),
        H(0x06, "h"),
        F(0x07, "f"),
    }
}

// impl_for!(for Register, as u8, impl usize);
//
// impl_str!(for Register);

// impl TryFrom<&str> for Register {
//     type Error = String;
//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         match value {
//             "a" => Ok(Self::A),
//             "b" => Ok(Self::B),
//             "c" => Ok(Self::C),
//             "d" => Ok(Self::D),
//             "z" => Ok(Self::Z),
//             "l" => Ok(Self::L),
//             "h" => Ok(Self::H),
//             "f" => Ok(Self::F),
//
//             x => Err(format!("Invalid register: {x:#?}")),
//         }
//     }
// }
