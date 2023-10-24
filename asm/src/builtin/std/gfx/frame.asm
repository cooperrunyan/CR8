; CALLER MUST SET mb 1
; Draws bytes from ROM to VRAM
#[macro] clrvram: {
    ($from: expr, $to: expr) => {
        mov %a, $from.l
        mov %b, $from.h
        mov %c, $to.l
        mov %d, $to.h
        call _clrvram
    }
}

; ab: From
; cd: To
_clrvram:
    sub %c, %d, %a, %b ; Length to clear
    mov %z, 0

    .loop:
        mov %x, %a
        mov %y, %b
        sw %z
        inc %a, %b
        dec %c, %d
        jnz .loop, %c
        jnz .loop, %d
        ret

; ab: Frame address
; cd: Write location
; [PSR0][PSR1]: Frame length
frmwof:
    .loop:
        mov %x, %a
        mov %y, %b
        lw %z
        inc %x, %y
        mov %a, %x
        mov %b, %y
        mov %x, %c
        mov %y, %d
        sw %z
        inc %x, %y
        push %x ; c
        push %y ; d
        lw %c, PSR0
        lw %d, PSR1
        dec %c, %d
        sw PSR0, %c
        sw PSR1, %d
        mov %z, %c
        or %z, %d
        pop %d
        pop %c
        jnz .loop, %z
        ret
