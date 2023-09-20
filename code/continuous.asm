#[use(std)]

#[static(LEN: 4)]

#[boot]
main:
    mb 1
    mov %a, 0
    mov %b, 0
    mov %c, [(0x4000 - 32) & 0xFF]
    mov %d, [(0x4000 - 32) >> 8]
    mov %z, 1

    .loop:
        ldhl [BRAM]
        add %l, %a
        add %h, %b
        mov %z, 0xff
        sw %z

        ldhl [BRAM]
        add %l, %c
        add %h, %d
        mov %z, 0
        sw %z

        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a
        mov %a, %a

        call [cycle]
        jmp [.loop]

cycle:
    inc %a, %b
    inc %c, %d
    and %b, 0x3f
    and %d, 0x3f
    ret
