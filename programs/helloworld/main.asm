#use "<std>/macro"
#use "<std>/gfx"
#use "<std>/wait"
#use "<std>"

#use "./images/HELLO"
#use "./images/WORLD"

#define WAIT 0x2000
#define OFFSET 0x0400

#init {
    mb 0x01
    jmp [hello]
}

hello:
    mov %a, %b, [HELLO]
    mov %c, %d, [BRAM]
    sw [PSR0], [HELLO_SZL]
    sw [PSR1], [HELLO_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM], [BRAM + HELLO_SZ]

    jmp [world]

world:
    mov %a, %b, [WORLD]
    mov %c, %d, [BRAM + OFFSET]
    sw [PSR0], [WORLD_SZL]
    sw [PSR1], [WORLD_SZH]
    call [frmwof]
    wait [WAIT]
    clrvram [BRAM + OFFSET], [BRAM + OFFSET + WORLD_SZ]

    jmp [hello]
