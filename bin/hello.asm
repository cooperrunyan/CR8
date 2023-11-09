#[use(std::gfx::text)]
#[use(std::sleep)]

#[main]
main:
    bank 1

    draw_char BRAM + 0x100 + 1, CHAR_H
    draw_char BRAM + 0x100 + 2, CHAR_E_LC
    draw_char BRAM + 0x100 + 3, CHAR_L_LC
    draw_char BRAM + 0x100 + 4, CHAR_L_LC
    draw_char BRAM + 0x100 + 5, CHAR_O_LC

    draw_char BRAM + 0x100 + 7, CHAR_W_LC
    draw_char BRAM + 0x100 + 8, CHAR_O_LC
    draw_char BRAM + 0x100 + 9, CHAR_R_LC
    draw_char BRAM + 0x100 + 10, CHAR_L_LC
    draw_char BRAM + 0x100 + 11, CHAR_D_LC

    .flip:
        mov %a, (BRAM + 0x100) & 0xFF
        mov %b, (BRAM + 0x100) >> 8
        mov %c, (0x100) & 0xFF
        mov %d, (0x100) >> 8
        call invert

        mov %a, 0
        mov %b, 60
        mov %c, 0
        mov %d, 0
        call sleep

        jmp .flip


; ab: Start
; cd: Length
invert:
    .loop:
        mov %x, %a
        mov %y, %b
        lw %z
        not %z
        sw %z
        inc %a, %b
        dec %c, %d
        jnz .loop, %c
        jnz .loop, %d
        ret
