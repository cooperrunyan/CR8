#[macro] not: {
    ($lhs: reg) => {
        nor $lhs, $lhs
    }
}

#[macro] nand: {
    ($lhs: reg, $rhs: any) => {
        and $lhs, $rhs
        not $lhs
    }
}
