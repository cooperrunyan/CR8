############################################################
# Clear
@macro
clrf:
  mov %f, 0


############################################################
# Arithmetic
@macro
sub $0, $1:
  clrf
  sbb $0, $1

@macro
add $0, $1:
  clrf
  adc $0, $1

@macro
inc $0:
  add $0, 0x1

@macro
dec $0:
  sub $0, 0x1

############################################################
# Logic
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

############################################################
# Control Flow
@macro
lda $0, $1:
  mov %l, $0
  mov %h, $1

@macro
jmp $0, $1:
  lda $0, $1
  jnz 0x1

@macro
jeq $0, $1:
  and %f, 0b0010
  lda $0, $1
  jnz %f

@macro
jz $0, $1, $2:
  cmp $2, 0
  jeq $0, $1

@macro
jlt $0, $1:
  and %f, 0b0001
  lda $0, $1
  jnz %f

@macro
jle $0, $1:
  and %f, 0b0011
  lda $0, $1
  jnz %f

@macro
jgt $0, $1:
  not %f
  and %f, 0b0001
  lda $0, $1
  jnz %f

@macro
jne $0, $1:
  not %f
  and %f, 0b0010
  lda $0, $1
  jnz %f

@macro
jge $0, $1:
  nand %f, 0b0001
  and %f, 0b0011
  lda $0, $1
  jnz %f


############################################################
# Calling
@macro
call $0, $1:
  push [(. + 13) >> 8]
  push [(. + 10) & 0x00FF]
  jmp $0, $1

@macro
ret:
  pop %l
  pop %h
  jnz 0x1

############################################################
# Devices
@macro
outb $0, $1:
  mov %d, $1
  out $0, %d

@macro
halt:
  outb DEV_CONTROL, SIGHALT

@macro
peekr:
  outb DEV_CONTROL, SIGDBG

@macro
peek $0:
  outb DEV_CONTROL, SIGPEEK
  outb DEV_CONTROL, $0

############################################################
