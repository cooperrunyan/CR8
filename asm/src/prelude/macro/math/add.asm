#[use(prelude::macro::clear)]

#[macro] add: {
    ($into: reg, $rhs: reg | imm8) => {
        clrfc
        adc $into, $rhs
    }
    ($tol: reg, $toh: reg, $frl: reg | imm8, $frh: reg | imm8) => {
        add $tol, $frl
        adc $toh, $frh
    }
    ($tol: reg, $toh: reg, $rhs: imm16) => {
        add $tol, $rhs.l
        adc $toh, $rhs.h
    }
}

#[macro] adc: {
    ($into: reg) => {
        adc $into, 0
    }
}

#[macro] inc: {
    ($into: reg) => {
        add $into, 1
    }
    ($lo: reg, $hi: reg) => {
        clrfb
        inc $lo
        adc $hi
    }
}
