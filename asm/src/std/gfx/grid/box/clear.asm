#[use(std::gfx::grid::point)]
#[use(std::gfx::grid::inline_box)]

; Clears a box 
clear_box:
    call [point_addr]

    ; Draw %ab
    inline_box 0, 0, 0, 0, 0, 0, 0, 0

    ret



