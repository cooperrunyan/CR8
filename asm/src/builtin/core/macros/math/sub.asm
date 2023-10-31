#[use(core::macros::clear)]

#[macro] sub: {
    ($into: reg, $rhs: any) => {
        clrfb
        sbb $into, $rhs
    }
    ($tol: reg, $toh: reg, $frl: any, $frh: any) => {
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

