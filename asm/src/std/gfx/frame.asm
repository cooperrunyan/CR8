#include "<std>/arch"
#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/add"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/gfx/frame

; CALLER MUST SET mb 1
; Draws bytes from ROM to VRAM
; Can be shortcut called with the macro: `frame [IMG], 128`

#macro
frame [a0, ir0]:
    mov %b, $a0h
    mov %a, $a0l
    mov %c, $ir0
    call [drawframe]

drawframe:
    push %b
    push %a
    push [BRAM >> 8]
    push [BRAM & 0x00FF]

    jnza [.loop], %c
    jmp [.done]

    .loop:
        dec %c
        pop %a
        pop %b
        pop %l
        pop %h
        lw %d
        inc16 %l, %h
        push %h
        push %l
        mov %l, %a
        mov %h, %b
        sw %d
        inc16 %l, %h
        push %h
        push %l
        jnza [.loop], %c
        jmp [.done]

    .done:
        pop %a
        pop %a
        pop %a
        pop %a
        ret
