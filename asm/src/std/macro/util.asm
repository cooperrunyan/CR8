;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/util

#macro movr16 (r0, r1, r2, r3) {
    mov $r0, $r2
    mov $r1, $r3
}

#macro mov16 (r0, r1, a0) {
    mov $r0, $a0l
    mov $r1, $a0h
}

#macro swi (a0, i0) {
    mov %f, $i0
    sw $a0, %f
}
