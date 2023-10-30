#[use(core::macros::jmp)]

#[macro] call: {
    ($addr: expr) => {
        push ($ + 7) >> 8     ; 2 bytes
        push ($ + 5) & 0x00FF ; 2 bytes
        jmp $addr             ; 3 bytes
    }
    ($l: lit | reg, $h: lit | reg) => {
        push ($ + 7) >> 8     ; 2 bytes
        push ($ + 5) & 0x00FF ; 2 bytes
        jmp $l, $h            ; 3 bytes
    }
}

#[macro] ret: {
    () => {
        pop %x
        pop %y
        jmp
    }
}
