#include "<std>/macro/clear"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/math/sub

#macro sub {
    ($into: reg, $rhs: reg | imm8) => {
        clrfb
        sbb $into, $rhs
    }
    ($into: reg) => {
        sbb $into, 0
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

#macro dec {
    ($into: reg) => {
        sub $into, 1
    }
    ($lo: reg, $hi: reg) => {
        clrfb
        dec $lo
        sub $hi
    }
}

