#[use(std::gfx::grid::box::clear)]
#[use(std::sleep)]

#[boot]
main:
    mov %mb, 1

    .loop:
        in %a, [RNG]
        in %b, [RNG]
        call [filled_box]

        ; ~ 1s
        mov %a, 0
        mov %b, 128
        mov %c, 0     
        mov %d, 0     

        call [sleep]

        jmp [.loop]

    
