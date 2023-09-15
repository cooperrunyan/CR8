#include "<std>/macro/clear"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/math/add

#macro
add [r0, ir0]:
    clrfc
    adc $r0, $ir0

#macro
add16 [r0, r1, ir0, ir1]:
    add $r0, $ir0
    adc $r1, $ir1

#macro
adcf [r0]:
    adc $r0, 0

#macro
inc [r0]:
    add $r0, 1

#macro
inc16 [r0, r1]:
    clrfc
    inc $r0
    adcf $r1

