#[use(core::macros::clear)]

#[macro] sub: {
    ($into: reg, $rhs: reg | lit) => {
        clrfb
        sbb $into, $rhs
    }
    ($tol: reg, $toh: reg, $frl: reg | lit, $frh: reg | lit) => {
        sub $tol, $frl
        sbb $toh, $frh
    }
    ($tol: reg, $toh: reg, $rhs: expr) => {
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
        dec $lo
        sbb $hi
    }
}

