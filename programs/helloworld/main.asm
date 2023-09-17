#include "<std>/macro"
#include "<std>/gfx"
#include "<std>/wait"
#include "<std>"

#include "./images/HELLO"
#include "./images/WORLD"

#define WAIT 0x2000
#define OFFSET 0x0400

#init {
    mb 0x01
    jmp [hello]
}

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
