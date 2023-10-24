#[macro] not: {
    ($lhs: reg) => {
        nor $lhs, $lhs
    }
}

#[macro] nand: {
    ($lhs: reg, $rhs: reg | lit) => {
        and $lhs, $rhs
        not $lhs
    }
}
