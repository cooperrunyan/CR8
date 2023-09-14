#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

main:
    mov %a, 12
    mov %b, 14
    call [mul]

    dbg

    halt

