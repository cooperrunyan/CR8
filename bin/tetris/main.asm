#[use(std)]

#[use("./draw")]

#[dyn(CURRENT: 8)]
#[dyn(OCCUPIED: 400)]

#[main]
main:
  bank 1
  mov %x, %y, BRAM
  call DrawRedTile
  mov %x, %y, BRAM + 4
  call DrawYellowTile
  mov %x, %y, BRAM + 8
  call DrawGreenTile
  mov %x, %y, BRAM + 12
  call DrawCyanTile
  mov %x, %y, BRAM + 16
  call DrawBlueTile
  mov %x, %y, BRAM + 20
  call DrawMagentaTile
  mov %x, %y, BRAM + 24
  call DrawOrangeTile
  jmp main

