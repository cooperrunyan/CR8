#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/sub"
#include "<std>/macro/math/add"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/math/mul/mul

; Multiply %a * %b -> %zd
mul:
    mov %z, 0
    jnza [.loop], %a
    ret

    .loop:
        dec %a
        add %z, %b
        add %d
        jnza [.loop], %a
        ret
