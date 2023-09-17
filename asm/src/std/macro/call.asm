#include "<std>/macro/jmp"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/call

#macro call (a0) {
    push [($ + 10) >> 8]
    push [($ + 8) & 0x00FF]
    jmp $a0
}

#macro ret () {
    pop %l
    pop %h
    jnz 1
}
