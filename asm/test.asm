#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

main:
    mov %a, 24
    mov %b, 12
    call [mul]
    halt


