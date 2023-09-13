#include "<std>/arch.asm"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Clear flags
#macro
clrf []:
  mov %f, 0

#macro
clrfb []:
  and %f, 0b0111

#macro
clrfc []:
  and %f, 0b1011

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Logic
#macro
nand [r0, ir0]:
  and $r0, $ir0
  not $r0

#macro
not [r0]:
  nor $r0, $r0

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Control Flow
#macro
ldhl [a0]:
  mov %l, $a0l
  mov %h, $a0h

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


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Calling
#macro
call [a0]:
  push [($ + 10) >> 8]
  push [($ + 8) & 0x00FF]
  jmp $a0

#macro
ret []:
  pop %l
  pop %h
  jnz 1

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Devices
#macro
outb [i0, i1]:
  push %f
  mov %f, $i1
  out $i0, %f
  pop %f

#macro
halt []:
  outb &SYSCTRL, &SIGHALT

#macro
peek [a0]:
  outb &SYSCTRL, &SIGPEEK
  outb &SYSCTRL, $a0l
  outb &SYSCTRL, $a0h

#macro
dbg []:
  outb &SYSCTRL, &SIGDBG

#macro
gpu [i0]:
  outb &GPU, $i0

#macro
render []:
  gpu &GPU_RENDER

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Math
#macro
add16 [r0, r1, ir0, ir1]:
    add %a, $ir0
    adc %b, $ir1

#macro
sub16 [r0, r1, ir0, ir1]:
    sub %a, $ir0
    sbb %b, $ir1

#macro
adcc [r0]:
    adc $r0, 0

#macro
sbbb [r0]:
    sbb $r0, 0

#macro
add [r0, ir0]:
    clrfc
    adc $r0, $ir0

#macro
sub [r0, ir0]:
    clrfb
    sbb $r0, $ir0

#macro
dec [r0]:
    sub $r0, 1

#macro
inc [r0]:
    add $r0, 1

#macro
dec16 [r0, r1]:
    clrfb
    dec $r0
    sbbb $r1

#macro
inc16 [r0, r1]:
    clrfc
    inc $r0
    adcc $r1
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
