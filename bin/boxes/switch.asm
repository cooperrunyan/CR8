#[use(std::gfx::grid::block::filled)]
#[use(std::gfx::grid::block::clear)]

#[boot]
main:
    mov %mb, 1

    push 0

    .loop:
        in %d, [KB]
        and %d, 0b00010000

        jnz [.switch], %d

        .finish_loop:
            pop %d
            push %d ; clone it in the stack
            jnz [.clear], %d
            jmp [.box]

        .clear:
            in %a, [RNG]
            in %b, [RNG]
            call [clear_box]
            jmp [.loop]

        .box:
            in %a, [RNG]
            in %b, [RNG]
            call [filled_box]
            jmp [.loop]

    .switch:
        pop %d
        not %d
        and %d, 1
        push %d
        jmp [.finish_loop]

