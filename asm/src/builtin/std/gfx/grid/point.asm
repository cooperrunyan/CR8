#[use(std::math::mul)]
#[use(std::gfx::grid::cfg)]

; 59 bytes long
; Calculate the address of a grid point (32x32 blocks)
; Mutates %a and %b as the return value
; Args: 
;   - %a:  x-value (0-31)
;   - %b:  y-value (0-31)
point_addr:
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
    mov %b, SCREEN_WIDTH
    call mulip ; %b * 32 -> %ab

    pop %z ; add %a 
    add %a, %b, %z, 0

    ; add the address of the base of banked ram
    add %a, %b, BRAM
    ret
