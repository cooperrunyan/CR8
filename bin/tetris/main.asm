#[use(std::gfx::grid::block)]
#[use(std::gfx::grid::point)]
#[use(std::sleep)]

#[static(ROW_LEN: 128)]
#[static(THICKNESS: 4)]
#[static(PAD_ROW: 5)]
#[static(PAD_COL: 5)]

#[static(ROWS: 20)]
#[static(COLS: 20)]

#[static(GREY: 0b10_10_10)]
#[static(CYAN: 0b00_11_11)]
#[static(YELLOW: 0b11_11_00)]
#[static(MAGENTA: 0b11_00_11)]

; I, O, T, S, Z, J, L
#[dyn(CURRENT_COLOR: 1)]
#[dyn(CURRENT_ROTATION: 1)]
#[dyn(CURRENT: 8)]

#[dyn(OCCUPIED: 400)]
; Each row is 20 bytes. 6 = top, 26 = bottom

#[main]
main:
    bank 1

    call draw_border

    call init_current


    .loop:
        call erase_current
        call tick
        call draw_current

        mov %a, 0
        mov %b, 32
        mov %c, 0
        mov %d, 0
        call sleep
        jmp .loop


init_current:
    mov %a, %b, 15, 6
    sw CURRENT, %a
    sw CURRENT + 1, %b
    inc %a
    sw CURRENT + 2, %a
    sw CURRENT + 3, %b
    inc %b
    sw CURRENT + 4, %a
    sw CURRENT + 5, %b
    dec %a
    sw CURRENT + 6, %a
    sw CURRENT + 7, %b

    sw CURRENT_ROTATION, 0
    sw CURRENT_COLOR, YELLOW
    ret

tick:
    ; Set Z to 1 if the current piece cannot go down
    call check_beneath_current

    jnz .stop, %z

    lw %b, CURRENT + 1
    inc %b
    sw CURRENT + 1, %b

    lw %b, CURRENT + 3
    inc %b
    sw CURRENT + 3, %b

    lw %b, CURRENT + 5
    inc %b
    sw CURRENT + 5, %b

    lw %b, CURRENT + 7
    inc %b
    sw CURRENT + 7, %b
    ret

    .stop:
        call move_to_occupied
        call init_current
        ret

move_to_occupied:
    lw %a, CURRENT
    lw %b, CURRENT + 1
    call occupy
    lw %a, CURRENT + 2
    lw %b, CURRENT + 3
    call occupy
    lw %a, CURRENT + 4
    lw %b, CURRENT + 5
    call occupy
    lw %a, CURRENT + 6
    lw %b, CURRENT + 7
    call occupy

occupy:
    push %a, %b
    sub %b, PAD_ROW + 2
    sub %a, PAD_COL + 2
    push %a
    mov %a, ROWS
    call mulip ; Multiply b * 20

    pop %z
    add %a, %z
    adc %b

    add %a, %b, OCCUPIED

    mov %x, %y, %a, %b
    sw 1
    pop %a, %b
    block GREY
    ret

check_beneath_current:
    lw %a, CURRENT
    lw %b, CURRENT + 1
    call check_beneath
    push %z
    lw %a, CURRENT + 2
    lw %b, CURRENT + 3
    call check_beneath
    push %z
    lw %a, CURRENT + 4
    lw %b, CURRENT + 5
    call check_beneath
    push %z
    lw %a, CURRENT + 6
    lw %b, CURRENT + 7
    call check_beneath
    pop %f
    or %z, %f
    pop %f
    or %z, %f
    pop %f
    or %z, %f
    ret

; %ab
; z -> 0 if nothing below, 1 if something
check_beneath:
    cmp %b, 26
    mov %z, 1
    req

    sub %b, PAD_ROW + 1
    sub %a, PAD_COL + 1
    push %a
    mov %a, ROWS
    call mulip ; Multiply b * 20

    pop %z
    add %a, %z
    adc %b

    add %a, %b, OCCUPIED

    mov %x, %y, %a, %b
    lw %z
    jnz .true, %z

    .false:
        mov %z, 0
        ret

    .true:
        mov %z, 1
        ret


draw_current:
    lw %d, CURRENT_COLOR
    lw %a, CURRENT
    lw %b, CURRENT + 1
    block %d
    lw %a, CURRENT + 2
    lw %b, CURRENT + 3
    block %d
    lw %a, CURRENT + 4
    lw %b, CURRENT + 5
    block %d
    lw %a, CURRENT + 6
    lw %b, CURRENT + 7
    block %d
    ret

erase_current:
    lw %a, CURRENT
    lw %b, CURRENT + 1
    block 0
    lw %a, CURRENT + 2
    lw %b, CURRENT + 3
    block 0
    lw %a, CURRENT + 4
    lw %b, CURRENT + 5
    block 0
    lw %a, CURRENT + 6
    lw %b, CURRENT + 7
    block 0
    ret

draw_border:
    mov %x, %y, BRAM + (ROW_LEN * PAD_ROW * THICKNESS) +  ROW_LEN + (PAD_COL * THICKNESS) + THICKNESS / 2
    mov %c, COLS * 4 + 4
    .top:
        dec %c
        sw 0b101010
        inc %x, %y
        jnz .top, %c

    dec %x, %y
    mov %c, (ROWS + 2) * 4
    .sides:
        dec %c
        add %x, %y, ROW_LEN - (COLS * 4 + 3)
        sw 0b101010
        add %x, %y, COLS * 4 + 3
        sw 0b101010
        jnz .sides, %c

    mov %x, %y, BRAM + (ROW_LEN * (32 - PAD_ROW) * THICKNESS) + (2 * ROW_LEN) + (PAD_COL * THICKNESS) + THICKNESS / 2
    mov %c, COLS * 4 + 4
    .bottom:
        dec %c
        sw 0b101010
        inc %x, %y
        jnz .bottom, %c

    ret



; Return if eq
#[macro] req: {
  () => {
    pop %x
    pop %y

    jeq

    push %y
    push %x
  }
}
