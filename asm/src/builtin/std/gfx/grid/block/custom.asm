#[use(std::gfx::grid::cfg)]

; Draws a box at %ab with the last 8 bytes pushed to the stack
custom_box:
    ; Pop the caller's address to access the items that 
    ; the caller pushed. 
    pop %c
    pop %d

    ldhl %a, %b
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    add %l, %h, [SCREEN_WIDTH], 0
    pop %z
    sw %z

    ; Put the caller's address back
    push %d
    push %c

    ret
