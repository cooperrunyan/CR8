#include "<std>/arch"
#include "<std>/macro/call"
#include "<std>/macro/jmp"
#include "<std>/macro/math/add"
#include "<std>/macro"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/gfx/frame

; CALLER MUST SET mb 1
; Draws bytes from ROM to VRAM

#macro clrvram {
    ($from: imm16, $to: imm16) => {
        mov %a, $from.l
        mov %b, $from.h
        mov %c, $to.l
        mov %d, $to.h
        call [_clrvram]
    }
}

; ab: From
; cd: To
_clrvram:
    sub %c, %d, %a, %b ; Length to clear
    mov %z, 0

    .loop:
        mov %l, %a
        mov %h, %b
        sw %z
        inc %a, %b
        dec %c, %d
        jnz [.loop], %c
        jnz [.loop], %d
        ret

; ab: Frame address
; cd: Write location
; [PSR0][PSR1]: Frame length
frmwof:
    .loop:
        mov %l, %a
        mov %h, %b
        lw %z
        inc %l, %h
        mov %a, %l
        mov %b, %h
        mov %l, %c
        mov %h, %d
        sw %z
        inc %l, %h
        push %l ; c
        push %h ; d
        lw %c, [PSR0]
        lw %d, [PSR1]
        dec %c, %d
        sw [PSR0], %c
        sw [PSR1], %d
        mov %z, %c
        or %z, %d
        pop %d
        pop %c
        jnz [.loop], %z
        ret
