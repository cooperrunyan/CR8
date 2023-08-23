;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Arithmetic
@macro
sub $0, $1:
  mov %f, 0b0000
  sbb $0, $1

@macro
add $0, $1:
  mov %f, 0b0000
  adc $0, $1

@macro
inc $0:
  add $0, 1

@macro
dec $0:
  sub $0, 1

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Logic
@macro
nand $0, $1:
  and $0, $1
  not $0

@macro
not $0:
  nor $0, $0

@macro
xor $0, $1:
  nor $0, $0

@macro
xnor $0, $1:
  nor $0, $0

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Control Flow
@macro
lda $0, $1:
  mov %l, $0
  mov %h, $1

@macro
jnza $0, $1, $2:
  lda $0, $1
  jnz $2

@macro
jmp $0, $1:
  jnza $0, $1, 1

@macro
jeq $0, $1:
  and %f, 0b0010
  jnza $0, $1, %f

@macro
jlt $0, $1:
  and %f, 0b0001
  jnza $0, $1, %f

@macro
jle $0, $1:
  and %f, 0b0011
  jnza $0, $1, %f

@macro
jgt $0, $1:
  not %f
  and %f, 0b0001
  jnza $0, $1, %f

@macro
jne $0, $1:
  not %f
  and %f, 0b0010
  jnza $0, $1, %f

@macro
jge $0, $1:
  nand %f, 0b0001
  and %f, 0b0011
  jnza $0, $1, %f


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; Calling
@macro
call $0, $1:
  push [(. + 11) >> 8]    ; 3 bytes
  push [(. + 8) & 0x00FF] ; 3 bytes
  jmp $0, $1             ; 5 bytes

@macro
ret:
  pop %l
  pop %h
  jnz 1

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
