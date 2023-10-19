#[use(core::macros::logic)]

#[macro] ldhl: {
    ($addr: expr) => {
        mov %h, $addr.h
        mov %l, $addr.l
    }
    ($l: lit | reg, $h: lit | reg) => {
        mov %h, $h
        mov %l, $l
    }
}

#[macro] jnz: {
    ($addr: expr, $if: lit | reg) => {
        ldhl $addr
        jnz $if
    }
    ($addr: expr, $ifl: reg, $ifh: reg) => {
        mov %f, $ifl
        or %f, $ifh
        jnz $addr, %f
    }
}

#[macro] jmp: {
    ($addr: expr) => {
        jnz $addr, 1
    }
    ($l: lit | reg, $h: lit | reg) => {
        ldhl $l, $h
        jmp
    }
    () => {
        jnz 1
    }
}

#[macro] jeq: {
    ($addr: expr) => {
        and %f, 0b0010
        jnz $addr, %f
    }
    ($addr: expr, $r: reg, $cmp: lit | reg) => {
        cmp $r, $cmp
        jeq $addr
    }
    () => {
      and %f, 0b0010
      jnz %f
    }
}

#[macro] jneq: {
    ($addr: expr) => {
        not %f
        and %f, 0b0010
        jnz $addr, %f
    }
    () => {
        not %f
        and %f, 0b0010
        jnz %f
    }
}

#[macro] jlt: {
    ($addr: expr) => {
        and %f, 0b0001
        jnz $addr, %f
    }
}

#[macro] jle: {
    ($addr: expr) => {
        and %f, 0b0011
        jnz $addr, %f
    }
}

#[macro] jgt: {
    ($addr: expr) => {
        not %f
        and %f, 0b0001
        jnz $addr, %f
    }
}

#[macro] jge: {
    ($addr: expr) => {
        nand %f, 0b0001
        and %f, 0b0011
        jnz $addr, %f
    }
}

#[macro] jz: {
    ($addr: expr, $if: reg) => {
        cmp $if, 0b0010
        jeq $addr
    }
}
