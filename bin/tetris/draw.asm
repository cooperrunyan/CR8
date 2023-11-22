
DrawGreyTile:
  _drawer 0b101010, 0b111111, 0b111111, 0b010101
  ret

DrawMagentaTile:
  _drawer 0b110011, 0b110111, 0b111011, 0b100010
  ret

DrawCyanTile:
  _drawer 0b001111, 0b011111, 0b101111, 0b001010
  ret

DrawYellowTile:
  _drawer 0b111100, 0b111101, 0b111110, 0b101000
  ret

DrawOrangeTile:
  _drawer 0b111000, 0b111001, 0b111001, 0b100100
  ret

DrawBlueTile:
  _drawer 0b000011, 0b010111, 0b101011, 0b000010
  ret

DrawRedTile:
  _drawer 0b110000, 0b110101, 0b111010, 0b100000
  ret

DrawGreenTile:
  _drawer 0b001100, 0b011101, 0b101110, 0b001000
  ret




#[macro] _drawer: {
  ($main: lit, $light: lit, $lighter: lit, $dark: lit) => {
    mov %a, $light
    sw %a
    inc %x, %y
    mov %a, $lighter
    sw %a
    inc %x, %y
    sw %a
    inc %x, %y
    sw %a
    add %x, %y, 125, 0
    mov %a, $light
    sw %a
    mov %a, $main
    inc %x, %y
    sw %a
    inc %x, %y
    sw %a
    mov %a, $dark
    inc %x, %y
    sw %a
    add %x, %y, 125, 0
    mov %a, $light
    sw %a
    mov %a, $main
    inc %x, %y
    sw %a
    inc %x, %y
    sw %a
    mov %a, $dark
    inc %x, %y
    sw %a
    add %x, %y, 125, 0
    mov %a, $light
    sw %a
    mov %a, $dark
    inc %x, %y
    sw %a
    inc %x, %y
    sw %a
    inc %x, %y
    sw %a
  }
}
