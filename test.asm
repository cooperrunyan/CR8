#include "asm/sys/sys.asm"
#define RAM 0x23
#origin 0x43

#dyn byte dynamic_byte1
#dyn 12 dynamic2

#mem byte rombt1 2
#mem 3 romdata1 [0x01, 0xF0, 0xF]


jmp [main]

sect:
    mov %b, 4

    .done:
    ret

main:
    call [sect]
    halt
