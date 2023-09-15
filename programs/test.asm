mb 0x01
jmp [main]

#include "<std>"
#include "./test.asm"

main:
    frame [TEST], [0x2000]

    mov %a, 0xff
    ldhl [0x9000]
    sw %a
    ldhl [0x8001]
    sw %a

    #marker debug
    dbg

    jmp [main]

    #marker halt
    halt

