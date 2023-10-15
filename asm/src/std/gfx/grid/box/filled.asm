#[use(std::gfx::grid::point)]
#[use(std::gfx::grid::inline_box)]

; 181 bytes long
; Draws %d as an 8x8 px box 
; Args: 
;   - %a:  x-value (0-31)
;   - %b:  y-value (0-31)
filled_box:
    call [point_addr]

    ; Draw %ab
    inline_box 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111

    ret
