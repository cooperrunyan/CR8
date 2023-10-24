#[use(std)]

#[static(LEN: 4)]

#[main]
main:
    mov %k, 1
    mov %a, 0
    mov %b, 0
    mov %c, (0x4000 - LEN) & 0xFF
    mov %d, (0x4000 - LEN) >> 8
    mov %z, 0xff

    .loop:
        ldxy BRAM
        add %x, %a
        add %y, %b
        sw %z

        call cycle
        jnz .loop, %a, %b
        not %z
        jmp .loop

cycle:
    inc %a, %b
    inc %c, %d
    and %b, 0x3f
    and %d, 0x3f
    ret
