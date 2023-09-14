#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

main:
    mov %a, 12
    mov %b, 14

    #marker call
    call [mul]

    #marker debug
    dbg

    #marker halt
    halt

