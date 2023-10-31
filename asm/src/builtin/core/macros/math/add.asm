#[use(core::macros::clear)]

#[macro] add: {
    ($into: reg, $rhs: any) => {
        clrfc
        adc $into, $rhs
    }
    ($tol: reg, $toh: reg, $frl: any, $frh: any) => {
        add $tol, $frl
        adc $toh, $frh
    }
    ($tol: reg, $toh: reg, $rhs: expr) => {
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
        inc $lo
        adc $hi
    }
}
