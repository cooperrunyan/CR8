#[use(std::gfx::grid::block::filled)]

#[main]
main:
    mov %mb, 1

    .loop:
        in %a, RNG
        in %b, RNG
        call filled_box

        jmp .loop
