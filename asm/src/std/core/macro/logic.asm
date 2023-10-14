#[macro] nand: {
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        nand $inl, $frl
        nand $inh, $frh
    }
}

#[macro] not: {
    ($inl: reg, $inh: reg) => {
        not $inl
        not $inh
    }
}

#[macro] xnor: {
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xnor $inl, $frl
        xnor $inh, $frh
    }
}


#[macro] xor: {
    ($inl: reg, $inh: reg, $frl: imm8 | reg, $frh: imm8 | reg) => {
        xor $inl, $frl
        xor $inh, $frh
    }
}
