#include "<std>/arch"
#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/add"
#include "<std>/macro"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/gfx/frame

; CALLER MUST SET mb 1
; Draws bytes from ROM to VRAM

#macro
clrvram [a0, a1]:
    mov %a, $a0l
    mov %b, $a0h
    mov %c, $a1l
    mov %d, $a1h
    call [_clrvram]

; ab: From
; cd: To
_clrvram:
    sub16 %c, %d, %a, %b ; Length to clear
    mov %z, 0

    .loop:
        mov %l, %a
        mov %h, %b
        sw %z
        inc16 %a, %b
        dec16 %c, %d
        jnza [.loop], %c
        jnza [.loop], %d
        ret

; ab: Frame address
; cd: Write location
; [PSR0][PSR1]: Frame length
frmwof:
    .loop:
        mov %l, %a
        mov %h, %b
        lw %z
        #marker inc
        inc16 %l, %h
        #marker moval
        mov %a, %l
        mov %b, %h
        #marker movlc
        mov %l, %c
        #marker movld
        mov %h, %d
        #marker movswz
        sw %z
        #marker incinh
        inc16 %l, %h
        #marker pushc
        push %l ; c
        #marker pushd
        push %h ; d
        lw %c, [PSR0]
        lw %d, [PSR1]
        dec16 %c, %d
        sw [PSR0], %c
        sw [PSR1], %d
        mov %z, %c
        or %z, %d
        pop %d
        pop %c
        jnza [.loop], %z
        ret
