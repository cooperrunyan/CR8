#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

main:
    mov %a, 24
    mov %b, 12
    call [mul]
    mb 0x01
    sw [BRAM], %d
    sw [BRAM + 1], %z
    mb 0x00
    halt

