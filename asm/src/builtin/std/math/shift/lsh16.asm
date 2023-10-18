; Logical Left Shift
; ab << c  -> ab
; Side effects: %a, %b, %c
lsh16:
    jnz .loop, %c
    ret

    .loop:
        dec %c
        add %a, %a
        adc %b, %b
        jnz .loop, %c
        ret

; Algorithmic Left Shift
; Side effects: %a, %b, %c, %d
lsa16:
    mov %d, %b
    call lsh16
    and %d, 0b10000000
    and %b, 0b01111111
    or %b, %d
    ret
