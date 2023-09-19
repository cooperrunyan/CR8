#[use(std::macro::call)]
#[use(std::macro::jmp)]
#[use(std::macro::math)]

; Logical Left Shift
; Side effects: %z, %b
lsh:
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
lsa:
    mov %d, %a
    and %d, 0b10000000
    call [lsh]
    or %z, %d
    ret

; Rotate left
; Side effects: %z, %b
lrt:
    mov %z, %a
    jnz [.loop], %b
    ret

    .loop:
        dec %b
        add %z, %z
        adc %z
        jnz [.loop], %b
        ret

