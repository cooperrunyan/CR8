#[use(std)]

; Draws %d as an 8x8 px box 
; Args: 
;   - %a:  x-value (0-31)
;   - %b:  y-value (0-31)
box:
    call [point_addr]

    ; Draw %ab
    box_at_addr 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111

    ret

; Draws a box with a border
bordered_box:
    call [point_addr]

    ; Draw %ab
    box_at_addr 0, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0

    ret

; Clears a box 
clear_box:
    call [point_addr]

    ; Draw %ab
    box_at_addr 0, 0, 0, 0, 0, 0, 0, 0

    ret

; Draws a box at the address: [%ab]
#[macro] box_at_addr: {
    (
      $l0: imm8 | reg,
      $l1: imm8 | reg,
      $l2: imm8 | reg,
      $l3: imm8 | reg,
      $l4: imm8 | reg,
      $l5: imm8 | reg,
      $l6: imm8 | reg,
      $l7: imm8 | reg) => {
        ldhl %a, %b
        sw $l0

        ; Draw next 7 lines (block height - 1)
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l1
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l2
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l3
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l4
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l5
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l6
        add %l, %h, [SCREEN_WIDTH], 0
        sw $l6
    }
}


; Calculate the address of a grid point (32x32 blocks)
; Mutates %a and %b as the return value
; Args: 
;   - %a:  x-value (0-31)
;   - %b:  y-value (0-31)
point_addr:
    #[static(BLOCK_HEIGHT: 8)]
    #[static(SCREEN_WIDTH: 32)] ; bytes -- 256 bits (px)

    ; set %a to the x position and %b to the y position (32 x 32), 
    ; multiply %b * 32, add %a, add BRAM and the 
    ; result is the starting address of the box.

    and %a, 0b00011111 ; constrain a to 0-31,

    ; make b a multiple of 8 (block height) by shifting left 3x
    add %b, %b
    add %b, %b
    add %b, %b
    push %a

    ; multiply %b * SCREEN_WIDTH
    mov %a, %b
    mov %b, [SCREEN_WIDTH]
    call [mulip] ; %b * 32 -> %ab

    pop %d ; add %a 
    add %a, %b, %d, 0

    ; add the address of the base of banked ram
    add %a, %b, [BRAM]
    ret
