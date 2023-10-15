#[use(std::gfx::grid::point)]
#[use(std::gfx::grid::inline_box)]

; Draws a box with a border
bordered_box:
    call [point_addr]

    ; Draw %ab
    inline_box 0, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0b01111110, 0

    ret
