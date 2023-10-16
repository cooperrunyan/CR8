#[use(std::gfx::grid::cfg)]

; Draws a box at the address: [%ab]
#[macro] inline_box: {
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
