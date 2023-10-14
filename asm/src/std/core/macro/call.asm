#[use(core::macro::jmp)]

#[macro] call: {
    ($addr: imm16) => {
        push [($ + 11) >> 8]    ; 2 bytes
        push [($ + 9) & 0x00FF] ; 2 bytes
        jmp $addr               ; 6 bytes
    }
    ($l: imm8 | reg, $h: imm8 | reg) => {
        push [($ + 11) >> 8]    ; 2 bytes
        push [($ + 9) & 0x00FF] ; 2 bytes
        jmp $l, $h              ; 6 bytes
    }
}

#[macro] ret: {
    () => {
        pop %l
        pop %h
        jmp
    }
}
