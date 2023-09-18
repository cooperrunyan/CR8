#include "<std>/macro/clear"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/math/sub

#macro sub {
    ($into: reg, $rhs: reg | imm8) => {
        clrfb
        sbb $into, $rhs
    }
}

#macro sub16 {
    ($intl: reg, $inth: reg, $rhsl: reg | imm8, $rhsh: reg | imm8) => {
        sub $intl, $rhsl
        sbb $inth, $rhsh
    }
}

#macro sbbf {
    ($into: reg) => {
        sbb $into, 0
    }
}

#macro dec {
    ($into: reg) => {
        sub $into, 1
    }
}

#macro dec16 {
    ($lo: reg, $hi: reg) => {
        clrfb
        dec $lo
        sbbf $hi
    }
}
