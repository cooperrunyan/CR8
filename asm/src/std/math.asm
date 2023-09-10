#include "<std>/macros.asm"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; MATH.asm

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Shift
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

; Shift ab << c   ->  cdz
lsh16:
    push %a
    push %c
    mov %z, %a
    mov %d, %b
    jnza [.loop], %c
    jmp [.done]

    .loop:
        dec %c
        add %z, %z
        adc %d, %d
        adcc %a
        jnza [.loop], %c
        jmp [.done]

    .done:
        mov %c, %a
        pop %c
        pop %a
        ret


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Multiply %a * %b -> %zd
mul:
    mov %z, 0
    jnza [.loop], %a
    ret

    .loop:
        dec %a
        add %z, %b
        adcc %d
        jnza [.loop], %a
        ret

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; 16 bit manipulation
; Multiply %ab * %cd -> %abcd
; Occupies PSR:
; Byte:  0   1   2   3
;       [0] [1]
;           [2] [3]
;           [4] [5]
;               [6] [7]
mul16:
    push %a
    push %b
    push %a
    push %b
    mov %z, %a
    mov %a, 0
    mov %b, 0

    ; Clear used PSRs
    sw [PSR0], %a
    sw [PSR1], %a
    sw [PSR2], %a
    sw [PSR3], %a
    sw [PSR4], %a
    sw [PSR5], %a
    sw [PSR6], %a
    sw [PSR7], %a

    jnza [.loop_00], %z
    jmp [.done_00]

    .loop_00:
        dec %z
        add %a, %c
        adcc %b
        jnza [.loop_00], %z
        jmp [.done_00]

    .done_00:
        sw [PSR0], %a
        sw [PSR1], %b
        pop %z
        mov %a, 0
        mov %b, 0
        jmp [.loop_01]

    .loop_01:
        dec %z
        add %a, %c
        adcc %b
        jnza [.loop_01], %z
        jmp [.done_01]

    .done_01:
        sw [PSR2], %a
        sw [PSR3], %b
        pop %z
        mov %a, 0
        mov %b, 0
        jnza [.loop_10], %z
        jmp [.done_10]

    .loop_10:
        dec %z
        add %a, %d
        adcc %b
        jnza [.loop_10], %z
        jmp [.done_10]

    .done_10:
        sw [PSR4], %a
        sw [PSR5], %b
        pop %z
        mov %a, 0
        mov %b, 0
        jnza [.loop_11], %z
        jmp [.done_11]

    .loop_11:
        dec %z
        add %a, %d
        adcc %b
        jnza [.loop_11], %z
        jmp [.done]

    .done_11:
        sw [PSR6], %a
        sw [PSR7], %b
        pop %z
        mov %a, 0
        mov %b, 0
        jnza [.loop_11], %z
        jmp [.done]

    .done:
        mov %d, 0
        lw %a, [PSR1]
        lw %b, [PSR2]
        lw %c, [PSR4]
        add %a, %b
        adcc %d
        add %a, %c
        adcc %d
        sw [PSR1], %a
        lw %a, [PSR3]
        lw %b, [PSR5]
        lw %c, [PSR6]
        add %a, %d
        mov %d, 0
        adcc %d
        add %a, %b
        adcc %d
        add %a, %c
        adcc %d
        mov %c, %a
        lw %a, [PSR7]
        add %d, %a
        lw %a, [PSR0]
        lw %b, [PSR1]
        ret
