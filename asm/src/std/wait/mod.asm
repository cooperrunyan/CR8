#include "<std>/macro/math/sub"
#include "<std>/macro/jmp"
#include "<std>/macro/call"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/wait

; Can be shortcut called with the macro: `wait [TICKS]`

#macro wait {
    ($tickamt: imm16) => {
        mov %b, $tickamt.h
        mov %a, $tickamt.l
        call [_wait]
    }
}

_wait:
    .loop:
        dec16 %a, %b
        jnza [.loop], %a
        jnza [.loop], %b
        ret