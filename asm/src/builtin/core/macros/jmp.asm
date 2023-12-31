#[use(core::macros::logic)]

#[macro] ldxy: {
    ($addr: expr) => {
        mov %y, $addr.h
        mov %x, $addr.l
    }
    ($l: any, $h: any) => {
        mov %y, $h
        mov %x, $l
    }
}

#[macro] jnz: {
    ($addr: expr, $ifl: reg, $ifh: reg) => {
        mov %f, $ifl
        or %f, $ifh
        jnz $addr, %f
    }
}

#[macro] jeq: {
    ($addr: expr) => {
        and %f, 0b0010
        jnz $addr, %f
    }
    ($addr: expr, $r: reg, $cmp: any) => {
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
