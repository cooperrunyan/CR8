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
lda [a0]:
  mov %l, $a0l
  mov %h, $a0h

#macro
jmp [a0]:
  lda $a0
  jnz 1

#macro
jeq [a0]:
  and %f, 0b0010
  lda $a0
  jnz %f

#macro
jz [a0, r0]:
  cmp $r0, 0
  jeq $a0

#macro
jlt [a0]:
  and %f, 0b0001
  lda $a0
  jnz %f

#macro
jle [a0]:
  and %f, 0b0011
  lda $a0
  jnz %f

#macro
jgt [a0]:
  not %f
  and %f, 0b0001
  lda $a0
  jnz %f

#macro
jne [a0]:
  not %f
  and %f, 0b0010
  lda $a0
  jnz %f

#macro
jge [a0]:
  nand %f, 0b0001
  and %f, 0b0011
  lda $a0
  jnz %f


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Calling
#macro
call [a0]:
  push [($ + 13) >> 8]
  push [($ + 10) & 0x00FF]
  jmp $a0

#macro
ret []:
  pop %l
  pop %h
  jnz 1

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Devices
#macro
outi [i0, i1]:
  mov %d, $i1
  out $i0, %d

#macro
halt []:
  outb &PORT_SIGNALLER, &SIGNAL_SIGHALT

#macro
peek [a0]:
  outb &PORT_SIGNALLER, &SIGNAL_SIGPEEK
  outb &PORT_SIGNALLER, $a0l
  outb &PORT_SIGNALLER, $a0h

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
