#define ROM       0x0000
#define BRAM      0x8000
#origin           0x8000
#define GPRAM     0xC000
#define STACK     0xFC00
#define STACK_END 0xFEFF


; Psuedo Register addresses
; Used for temporary data
#define PSR0 0xFF00
#define PSR1 0xFF01
#define PSR2 0xFF02
#define PSR3 0xFF03
#define PSR4 0xFF04
#define PSR5 0xFF05
#define PSR6 0xFF06
#define PSR7 0xFF07
#define PSR8 0xFF08
#define PSR9 0xFF09

#define SYSCTRL  0x00
#define SIGNOP   0x00
#define SIGHALT  0x01
#define SIGPEEK  0x02
#define SIGDBG   0x03
