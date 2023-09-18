;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/logic

#macro nand {
    ($into: reg, $rhs: imm8 | reg) => {
        and $into, $rhs
        not $into
    }
}

#macro not {
    ($into: reg) => {
        nor $into, $into
    }
}

#macro xnor {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $into
        nor $into, $rhs
        and %f, $rhs
        or $into, %f
    }
}

#macro xor {
    ($into: reg, $rhs: imm8 | reg) => {
        mov %f, $rhs
        or %f, $into
        nand $into, $rhs
        and $into, %f
    }
}
