#[macro] nand: {
    ($into: reg, $rhs: imm8 | reg) => {
        and $into, $rhs
        not $into
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        nand $inl, $frl
        nand $inh, $frh
    }
}

#[macro] not: {
    ($into: reg) => {
        nor $into, $into
    }
    ($inl: reg, $inh: reg) => {
        not $inl
        not $inh
    }
}

#[macro] xnor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $into
        nor $into, $rhs
        and %f, $rhs
        or $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xnor $inl, $frl
        xnor $inh, $frh
    }
}


#[macro] xor: {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $rhs
        or %f, $into
        nand $into, $rhs
        and $into, %f
    }
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xor $inl, $frl
        xor $inh, $frh
    }
}
