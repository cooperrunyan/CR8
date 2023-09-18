#use "<std>/macro/call"
#use "<std>/macro/jmp"
#use "<std>/macro/math"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/math/shift/lsh

; Logical Left Shift
; Side effects: %z, %b
lshl:
    mov %z, %a
    jnz [.loop], %b
    ret

    .loop:
        dec %b
        add %z, %z
        jnz [.loop], %b
        ret

; Algorithmic Left Shift
; Side effects: %z, %b, %d
lsha:
    mov %d, %a
    and %d, 0b10000000
    call [lshl]
    or %z, %d
    ret

; Rotate left
; Side effects: %z, %b
lshr:
    mov %z, %a
    jnz [.loop], %b
    ret

    .loop:
        dec %b
        add %z, %z
        adc %z
        jnz [.loop], %b
        ret

