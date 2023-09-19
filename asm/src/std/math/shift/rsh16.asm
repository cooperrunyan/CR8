#[use(std::macro::call)]
#[use(std::macro::jmp)]
#[use(std::macro::math)]
#[use(std::math::shift::lsh16)]

; Rotate Right
; Side effects: %a, %b, %c
rshr16:
    ; Left rotate %ab (8 - %c) times
    mov %f, 16
    sub %f, %c
    and %f, 0b1111
    mov %c, %f
    call [lshr16]
    ret

; Logical Right Shift
; Side effects: %a, %b, %c
rshl16:
    push %c
    call [rshr16]
    pop %c
    push %b
    push %a
    mov %b, %c
    mov %c, 16
    sub %c, %b
    and %c, 0b1111
    mov %a, 0
    mov %b, 0
    jnz [.mask], %c
    jmp [.done]

    .mask:
        dec %c
        add %a, %a  ; Single left-shift
        adc %b, %b
        or %a, 1
        jnz [.mask], %c
        jmp [.done]

    .done:
        pop %c
        and %a, %c
        pop %c
        and %b, %c
        ret

; Algorithmic Right Shift
; Side effects: %a, %b, %c
rsha16:
    push %b
    call [rshl16]
    pop %c
    and %c, 0b10000000
    or %b, %c
    ret
