#[use(std::gfx::grid::block::filled)]

#[boot]
main:
    mov %mb, 1

    .loop:
        in %a, RNG
        in %b, RNG
        call filled_box

        jmp .loop
