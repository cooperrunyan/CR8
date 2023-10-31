#[use(std::gfx::grid::cfg)]

; Draws a box at the address: [%ab]
#[macro] inline_box: {
    (
      $l0: any,
      $l1: any,
      $l2: any,
      $l3: any,
      $l4: any,
      $l5: any,
      $l6: any,
      $l7: any) => {
        ldxy %a, %b
        sw $l0

        ; Draw next 7 lines (block height - 1)
        add %x, %y, SCREEN_WIDTH
        sw $l1
        add %x, %y, SCREEN_WIDTH
        sw $l2
        add %x, %y, SCREEN_WIDTH
        sw $l3
        add %x, %y, SCREEN_WIDTH
        sw $l4
        add %x, %y, SCREEN_WIDTH
        sw $l5
        add %x, %y, SCREEN_WIDTH
        sw $l6
        add %x, %y, SCREEN_WIDTH
        sw $l7
    }
}
