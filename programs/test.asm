@data byte x
@data word y

jmp [main]


mul:
  mov %z, 0
  lda [_mul_loop]
  jnz %a
  ret

  _mul_loop:
    dec %a
    add %z, %b
    lda [_mul_loop]
    jnz %a
    ret

main:
  mov %a, 12
  mov %b, 14
  call [mul]
  halt
