#[use(std)]
#[use("../lib/box")]

#[dyn(X: 1)]
#[dyn(Y: 1)]

#[boot]
main:
    mov %mb, 1
    sw [BRAM + 0x2000 - 1], 255

    .loop:
        jmp [.loop]
