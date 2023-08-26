

mul:
    push %ax
    mov %zx, 0
    mov %dx, 0
    clrf
    jnza [_mul_loop], %ax
    jmp [_mul_done]

    _mul_loop:
        dec %ax
        add %zx, %bx
        adc %dx, 0
        jnza [_mul_loop], %ax
        jmp [_mul_done]

    _mul_done:
        pop %ax
        ret

add16:
    add %ax, %cx
    adc %bx, %dx
    ret

