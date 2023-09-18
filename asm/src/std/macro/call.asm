#include "<std>/macro/jmp"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/call

#macro call {
    ($addr: imm16) => {
        push [($ + 10) >> 8]
        push [($ + 8) & 0x00FF]
        jmp $addr
    }
}

#macro ret {
    () => {
        pop %l
        pop %h
        jnz 1
    }
}
