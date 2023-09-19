#[use(std::macro::clear)]

#[macro] sub: {
    ($into: reg, $rhs: reg | imm8) => {
        clrfb
        sbb $into, $rhs
    }
    ($tol: reg, $toh: reg, $frl: reg | imm8, $frh: reg | imm8) => {
        sub $tol, $frl
        sbb $toh, $frh
    }
    ($tol: reg, $toh: reg, $rhs: imm16) => {
        sub $tol, $rhs.l
        sbb $toh, $rhs.h
    }
}

#[macro] sbb: {
    ($into: reg) => {
        sbb $into, 0
    }
}

#[macro] dec: {
    ($into: reg) => {
        sub $into, 1
    }
    ($lo: reg, $hi: reg) => {
        clrfb
        dec $lo
        sbb $hi
    }
}

