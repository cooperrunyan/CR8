#[use(std::gfx::grid)]

#[main]
main:
    mov %mb, 1
    mov %a, %b, 0, 0

    .loop:
        push %a, %b
        call bordered_box
        pop %a, %b
        inc %a
        and %a, 0b00011111

        jnz .loop, %a
        inc %b
        and %b, 0b00011111
        jmp .loop
