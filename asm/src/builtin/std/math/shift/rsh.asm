#[use(std::math::shift::lsh)]

; Rotate Right
; Side effects: %z, %b, %c
rrt:
    ; Left rotate %a (8 - %b) times
    mov %c, 8
    sub %c, %b
    and %c, 0b1111

    mov %b, %c
    call lrt
    ret

; Logical Right Shift
; Side effects: %z, %b, %c, %d
rsh:
    mov %d, 0
    mov %c, %b
    sub %c, 8
    and %c, 0b1111
    jnz .mask, %c
    mov %z, %a
    ret

    .mask:
        dec %c
        add %d, %d  ; Single left-shift
        or %d, 1
        jnz .mask, %c
        jmp .shift

    .shift:
        call rrt
        and %z, %d
        ret
