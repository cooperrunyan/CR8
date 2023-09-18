#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/sub"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/math/shift/lsh

lsh:
    push %b
    mov %z, %a
    jnz [.loop], %b
    jmp [.done]

    .loop:
        dec %b
        add %z, %z
        jnz [.loop], %b
        jmp [.done]

    .done:
        pop %b
        ret
