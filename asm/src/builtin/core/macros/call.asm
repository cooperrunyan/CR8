#[use(core::macros::jmp)]

#[macro] call: {
    ($addr: expr) => {
        push ($ + 11) >> 8    ; 2 bytes
        push ($ + 9) & 0x00FF ; 2 bytes
        jmp $addr               ; 6 bytes
    }
    ($l: lit | reg, $h: lit | reg) => {
        push ($ + 11) >> 8    ; 2 bytes
        push ($ + 9) & 0x00FF ; 2 bytes
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
