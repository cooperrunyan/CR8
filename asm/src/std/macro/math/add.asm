#include "<std>/macro/clear"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/math/add

#macro add {
    ($into: reg, $rhs: reg | imm8) => {
        clrfc
        adc $into, $rhs
    }
}

#macro add16 {
    ($intl: reg, $inth: reg, $rhsl: reg | imm8, $rhsh: reg | imm8) => {
        add $intl, $rhsl
        adc $inth, $rhsh
    }
}

#macro adcf {
    ($into: reg) => {
        adc $into, 0
    }
}

#macro inc {
    ($into: reg) => {
        add $into, 1
    }
}

#macro inc16 {
    ($lo: reg, $hi: reg) => {
        clrfb
        inc $lo
        adcf $hi
    }
}
