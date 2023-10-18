#[use(std::gfx::grid::cfg)]

; Draws a box at the address: [%ab]
#[macro] inline_box: {
    (
      $l0: lit | reg,
      $l1: lit | reg,
      $l2: lit | reg,
      $l3: lit | reg,
      $l4: lit | reg,
      $l5: lit | reg,
      $l6: lit | reg,
      $l7: lit | reg) => {
        ldhl %a, %b
        sw $l0

        ; Draw next 7 lines (block height - 1)
        add %l, %h, SCREEN_WIDTH 
        sw $l1
        add %l, %h, SCREEN_WIDTH
        sw $l2
        add %l, %h, SCREEN_WIDTH
        sw $l3
        add %l, %h, SCREEN_WIDTH
        sw $l4
        add %l, %h, SCREEN_WIDTH
        sw $l5
        add %l, %h, SCREEN_WIDTH
        sw $l6
        add %l, %h, SCREEN_WIDTH
        sw $l6
    }
}
