mb 0x01
jmp [hello]

#include "<std>/macro"
#include "<std>/gfx"
#include "<std>/wait"

#include "programs/helloworld/images/hello.asm"
#include "programs/helloworld/images/world.asm"

hello:
    mov16 %a, %b, [HELLO]
    mov16 %c, %d, [BRAM]
    swi [PSR0], 0x40
    swi [PSR1], 0x06
    call [frmwof]
    wait [0x2000]
    clrvram [BRAM], [BRAM + 0x0640]

    jmp [world]

world:
    mov16 %a, %b, [WORLD]
    mov16 %c, %d, [BRAM + 0x0400]
    swi [PSR0], 0x40
    swi [PSR1], 0x06
    call [frmwof]
    wait [0x2000]
    clrvram [BRAM + 0x0400], [BRAM + 0x0400 + 0x0640]

    jmp [hello]
