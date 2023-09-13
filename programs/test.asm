#include "<std>/arch.asm"
#include "<std>/macros.asm"

jmp [main]

#include "<std>/math.asm"

main:
    render
    jmp [main]

    halt

