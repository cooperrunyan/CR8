mb 0x01
jmp [hello]

#include "<std>/macro"
#include "<std>/gfx"
#include "<std>/wait"

#include "programs/helloworld/images/HELLO.asm"
#include "programs/helloworld/images/WORLD.asm"

#define WAIT 0x2000
#define OFFSET 0x0400

hello:
    mov16 %a, %b, [HELLO]
    mov16 %c, %d, [BRAM]
    swi [PSR0], [HELLO_SZL]
    swi [PSR1], [HELLO_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM], [BRAM + HELLO_SZ]

    jmp [world]

world:
    mov16 %a, %b, [WORLD]
    mov16 %c, %d, [BRAM + OFFSET]
    swi [PSR0], [WORLD_SZL]
    swi [PSR1], [WORLD_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM + OFFSET], [BRAM + OFFSET + WORLD_SZ]

    jmp [hello]
