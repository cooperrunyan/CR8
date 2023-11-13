#[use(std::math::mul)]
#[use(std::math::shift)]
#[use(std::gfx::grid::cfg)]

; Calculate the address of a grid point (32x32 blocks)
; Mutates %a and %b as the return value
; Args:
;   - %a:  x-value (0-31)
;   - %b:  y-value (0-31)
point_addr:
    add %a, %a
    add %a, %a

    add %b, %b

    add %a, %b, BRAM

    ret
