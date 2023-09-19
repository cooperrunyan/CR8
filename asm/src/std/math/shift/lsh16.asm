#[use(std::macro::call)]
#[use(std::macro::jmp)]
#[use(std::macro::math)]

; Logical Left Shift
; ab << c  -> ab
; Side effects: %a, %b, %c
lshl16:
    jnz [.loop], %c
    ret

    .loop:
        dec %c
        add %a, %a
        adc %b, %b
        jnz [.loop], %c
        ret

; Algorithmic Left Shift
; Side effects: %a, %b, %c, %d
lsha16:
    mov %d, %b
    call [lshl16]
    and %d, 0b10000000
    or %b, %d
    ret

; Rotate left
; Side effects: %a, %b, %c
lshr16:
    jnz [.loop], %c
    ret

    .loop:
        dec %c
        add %a, %a
        adc %b, %b
        adc %a
        jnz [.loop], %c
        ret
