#[use(std)]
#[use("../lib/box")]

#[boot]
main:
    mov %mb, 1

    .loop:
        in %a, [RNG]
        in %b, [RNG]
        call [box]

        ; ~ 1s
        mov %a, 0
        mov %b, 128
        mov %c, 0     
        mov %d, 0     

        call [sleep]

        jmp [.loop]

    
