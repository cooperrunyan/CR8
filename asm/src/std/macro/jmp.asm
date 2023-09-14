#include "<std>/macro/logic"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/jmp

#macro
ldhl [a0]:
  mov %h, $a0h
  mov %l, $a0l

#macro
jnza [a0, ir0]:
  ldhl $a0
  jnz $ir0

#macro
jmp [a0]:
  ldhl $a0
  jnz 1

#macro
jeq [a0]:
  and %f, 0b0010
  ldhl $a0
  jnz %f

#macro
jz [a0, r0]:
  cmp $r0, 0
  jeq $a0

#macro
jlt [a0]:
  and %f, 0b0001
  ldhl $a0
  jnz %f

#macro
jle [a0]:
  and %f, 0b0011
  ldhl $a0
  jnz %f

#macro
jgt [a0]:
  not %f
  and %f, 0b0001
  ldhl $a0
  jnz %f

#macro
jne [a0]:
  not %f
  and %f, 0b0010
  ldhl $a0
  jnz %f

#macro
jge [a0]:
  nand %f, 0b0001
  and %f, 0b0011
  ldhl $a0
  jnz %f
