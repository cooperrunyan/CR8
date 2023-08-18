@macro jnz %r0 %r1 %r2:
  mov %l, %r1
  mov %h, %r2
  jnz %r0

@macro jmp %r1 %r2:
  jnz $1D, %r1, %r2

