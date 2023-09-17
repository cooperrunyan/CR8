;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/logic

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Logic
#macro nand (r0, ir0) {
    and $r0, $ir0
    not $r0
}

#macro not (r0): nor $r0, $r0

#macro xnor (r0, ir0) {
    mov %f, $r0
    nor $r0, $ir0
    and %f, $ir0
    or $r0, %f
}

#macro xor (r0, ir0) {
    mov %f, $ir0
    or %f, $r0
    nand $r0, $ir0
    and $r0, %f
}
