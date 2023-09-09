#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

sect:
    mov %b, 4
    jmp [.done]

    .done:
        halt

main:
    jmp [sect]
