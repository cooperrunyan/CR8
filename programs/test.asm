@mem word x

main:
  mov %a, 32
  mov %b, 2
  lda [x]
  sw %a
  lda [x + 1]
  sw %b
  mov %a, 0
  mov %b, 0
  lda [x]
  lw %a
  lda [x + 1]
  lw %b
  halt
