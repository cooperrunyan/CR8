@use "./macros.asm"

@def byte REFERENCE_NAME = $12D
@def word REFERENCE_NAME = [`1 + 2`] ; comment
@def dble REFERENCE_NAME = &REF

@store byte name
@store dble name2
@store word name3

@macro
jmp %r0 %r1:
  mov %l %r0
  mov %h %r1
  jnz $1D

@macro
jmp:
  jnz $1D

@macro
jmp %i0:
  jnz $1D

main:
  mov %a %b
  lw  %a  $0x333; comment
  lw %b  $12
  pop %z
  jnz $0x20 ``
  add! %a %b
  push %z [&REFERENCE_NAME]
