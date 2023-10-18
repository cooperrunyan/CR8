; Multiply %a * %b -> %zd
mul:
    mov %z, 0
    jnz .loop, %a
    ret

    .loop:
        dec %a
        add %z, %b
        adc %d
        jnz .loop, %a
        ret

; Multiply %a * %b -> %ab
; In-place
mulip:
    mov %c, %b
    mov %z, %a
    mov %a, 0
    mov %b, 0
    jnz .loop, %c
    ret

    .loop:
        dec %c
        add %a, %z
        adc %b
        jnz .loop, %c
        ret
