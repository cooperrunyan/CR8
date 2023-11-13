#[use(std::gfx::grid::cfg)]

; Draws a box at the address: [%ab]
#[macro] raw_block: {
    ($color: any) => {
        ldxy %a, %b
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color

        add %x, %y, 125, 0
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color

        add %x, %y, 125, 0
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color

        add %x, %y, 125, 0
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color
        inc %x, %y
        sw $color
    }
}

#[macro] block: {
    ($color: any) => {
        call point_addr
        raw_block $color
    }
}
