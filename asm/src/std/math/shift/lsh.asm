#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/dec"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/math/shift/lsh

lsh:
    push %b
    mov %z, %a
    jnza [.loop], %b
    jmp [.done]

    .loop:
        dec %b
        add %z, %z
        jnza [.loop], %b
        jmp [.done]

    .done:
        pop %b
        ret
