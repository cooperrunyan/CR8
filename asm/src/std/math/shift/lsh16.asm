#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/add"
#include "<std>/macro/math/sub"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/math/shift/lsh16

; Shift ab << c  into  cdz
lsh16:
    push %a
    push %c
    mov %z, %a
    mov %d, %b
    jnz [.loop], %c
    jmp [.done]

    .loop:
        dec %c
        add %z, %z
        adc %d, %d
        adc %a
        jnz [.loop], %c
        jmp [.done]

    .done:
        mov %c, %a
        pop %c
        pop %a
        ret

