#[use(std::macro::call)]
#[use(std::macro::jmp)]
#[use(std::macro::math)]

; Multiply %a * %b -> %zd
mul:
    mov %z, 0
    jnz [.loop], %a
    ret

    .loop:
        dec %a
        add %z, %b
        adc %d
        jnz [.loop], %a
        ret
