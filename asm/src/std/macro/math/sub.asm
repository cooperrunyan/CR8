#include "<std>/macro/clear"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/math/sub

#macro
sub [r0, ir0]:
    clrfb
    sbb $r0, $ir0

#macro
sub16 [r0, r1, ir0, ir1]:
    sub %a, $ir0
    sbb %b, $ir1

#macro
sbbf [r0]:
    sbb $r0, 0

#macro
dec [r0]:
    sub $r0, 1

#macro
dec16 [r0, r1]:
    clrfb
    dec $r0
    sbbf $r1
