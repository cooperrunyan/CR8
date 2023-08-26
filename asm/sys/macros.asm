############################################################
# Clear
@macro
clrf:
  mov %fx, 0


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
  mov %lx, $0
  mov %hx, $1

@macro
jmp $0, $1:
  lda $0, $1
  jnz 0x1

@macro
jeq $0, $1:
  and %fx, 0b0010
  lda $0, $1
  jnz %fx

@macro
jz $0, $1, $2:
  cmp $2, 0
  jeq $0, $1

@macro
jlt $0, $1:
  and %fx, 0b0001
  lda $0, $1
  jnz %fx

@macro
jle $0, $1:
  and %fx, 0b0011
  lda $0, $1
  jnz %fx

@macro
jgt $0, $1:
  not %fx
  and %fx, 0b0001
  lda $0, $1
  jnz %fx

@macro
jne $0, $1:
  not %fx
  and %fx, 0b0010
  lda $0, $1
  jnz %fx

@macro
jge $0, $1:
  nand %fx, 0b0001
  and %fx, 0b0011
  lda $0, $1
  jnz %fx


############################################################
# Calling
@macro
call $0, $1:
  push [($@ + 13) >> 8]
  push [($@ + 10) & 0x00FF]
  jmp $0, $1

@macro
ret:
  pop %lx
  pop %hx
  jnz 0x1

############################################################
# Devices
@macro
outb $0, $1:
  mov %dx, $1
  out $0, %dx

@macro
halt:
  outb PORT_CONTROLLER, CONTROL_SIGHALT

@macro
peekr:
  outb PORT_CONTROLLER, CONTROL_SIGDBG

@macro
peek $0, $1:
  outb PORT_CONTROLLER, CONTROL_SIGPEEK
  outb PORT_CONTROLLER, $0
  outb PORT_CONTROLLER, $1

############################################################
