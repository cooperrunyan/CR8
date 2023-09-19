#[use(std::macro::call)]
#[use(std::macro::jmp)]
#[use(std::macro::math)]
#[use(std::math::shift::lsh)]

; Rotate Right
; Side effects: %z, %b, %c
rshr:
    ; Left rotate %a (8 - %b) times
    mov %c, 8
    sub %c, %b
    and %c, 0b111

    mov %b, %c
    call [lshr]
    ret

; Logical Right Shift
; Side effects: %z, %b, %c, %d
rshl:
    mov %d, 0
    mov %c, %b
    and %c, 0b111
    jnz [.mask], %c
    mov %z, %a
    ret

    .mask:
        dec %c
        add %d, %d  ; Single left-shift
        or %d, 1
        jnz [.mask], %c
        jmp [.shift]

    .shift:
        call [rshr]
        and %z, %d
        ret

; Algorithmic Right Shift
; Side effects: %z, %b, %c, %d
rsha:
    push %a
    call [rshl]
    pop %d
    and %d, 0b10000000
    or %z, %d
    ret
