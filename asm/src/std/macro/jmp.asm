#[use(std::macro::logic)]

#[macro] ldhl: {
    ($addr: imm16) => {
        mov %h, $addr.h
        mov %l, $addr.l
    }
    ($l: imm8 | reg, $h: imm8 | reg) => {
        mov %h, $h
        mov %l, $l
    }
}

#[macro] jnz: {
    ($addr: imm16, $if: imm8 | reg) => {
        ldhl $addr
        jnz $if
    }
    ($addr: imm16, $ifl: reg, $ifh: reg) => {
        mov %f, $ifl
        or %f, $ifh
        jnz $addr, %f
    }
}

#[macro] jmp: {
    ($addr: imm16) => {
        jnz $addr, 1
    }
    ($l: imm8 | reg, $h: imm8 | reg) => {
        ldhl $l, $h
        jmp
    }
    () => {
        jnz 1
    }
}

#[macro] jeq: {
    ($addr: imm16) => {
        and %f, 0b0010
        jnz $addr, %f
    }
}

#[macro] jlt: {
    ($addr: imm16) => {
        and %f, 0b0001
        jnz $addr, %f
    }
}

#[macro] jle: {
    ($addr: imm16) => {
        and %f, 0b0011
        jnz $addr, %f
    }
}

#[macro] jgt: {
    ($addr: imm16) => {
        not %f
        and %f, 0b0001
        jnz $addr, %f
    }
}

#[macro] jge: {
    ($addr: imm16) => {
        nand %f, 0b0001
        and %f, 0b0011
        jnz $addr, %f
    }
}

#[macro] jne: {
    ($addr: imm16) => {
        not %f
        and %f, 0b0010
        jnz $addr, %f
    }
}

#[macro] jz: {
    ($addr: imm16, $if: reg) => {
        cmp $if, 0b0010
        jeq $addr
    }
}
