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
