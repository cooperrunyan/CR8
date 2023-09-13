use util_macros::encodable;

encodable! {
    pub enum Operation {
        MOV(0x00, "mov"),
        LW(0x01, "lw"),
        SW(0x02, "sw"),
        PUSH(0x03, "push"),
        POP(0x04, "pop"),
        JNZ(0x05, "jnz"),
        IN(0x06, "in"),
        OUT(0x07, "out"),
        CMP(0x08, "cmp"),
        ADC(0x09, "adc"),
        SBB(0x0A, "sbb"),
        OR(0x0B, "or"),
        NOR(0x0C, "nor"),
        AND(0x0D, "and"),
        MB(0x0E, "mb"),
    }
}
