#[use(std::gfx::text)]

#[main]
main:
    mov %k, 1
    draw_char BRAM, CHAR_H
    draw_char BRAM + 1, CHAR_E_LC
    draw_char BRAM + 2, CHAR_L_LC
    draw_char BRAM + 3, CHAR_L_LC
    draw_char BRAM + 4, CHAR_O_LC

    draw_char BRAM + 6, CHAR_W_LC
    draw_char BRAM + 7, CHAR_O_LC
    draw_char BRAM + 8, CHAR_R_LC
    draw_char BRAM + 9, CHAR_L_LC
    draw_char BRAM + 10, CHAR_D_LC

    .loop:
        jmp .loop
