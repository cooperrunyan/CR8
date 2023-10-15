#[use(std)]
#[use("../lib/box")]

#[boot]
main:
    mov %mb, 1

    .loop:
        in %a, [RNG]
        in %b, [RNG]
        call [box]

        jmp [.loop]
