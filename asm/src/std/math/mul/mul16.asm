#[use(std::math::mul::mul)]

; 16 bit manipulation
; Multiply %ab * %cd -> %abcd
; Occupies PSR:
; Byte:  0   1   2   3
;       [0] [1]
;           [2] [3]
;           [4] [5]
;               [6] [7]

; Byte:  0    1    2    3
;       [ac] [ac]
;            [ad] [ad]
;            [bc] [bc]
;                 [bd] [bd]
mul16:
    push %d
    push %c
    push %b
    push %d
    mov %d, 0
    sw [PSR0], %d
    sw [PSR1], %d
    sw [PSR2], %d
    sw [PSR3], %d
    sw [PSR4], %d
    sw [PSR5], %d
    sw [PSR6], %d
    sw [PSR7], %d
    mov %b, %c
    call [mulip] ; puts original %a into %z
    sw [PSR0], %a
    sw [PSR1], %b
    pop %b ; original %d
    mov %a, %z
    call [mulip]
    sw [PSR2], %a
    sw [PSR3], %b
    pop %a ; original %b
    pop %b ; original %c
    call [mulip] ; puts %a (original %b) into %z
    sw [PSR4], %a
    sw [PSR5], %b
    mov %a, %z
    pop %b ; original %d
    call [mulip]
    sw [PSR6], %a
    sw [PSR7], %b
    lw %a, [PSR1]
    lw %b, [PSR2]
    mov %c, 0
    add %a, %b
    adc %c
    lw %b, [PSR4]
    add %a, %b
    adc %c
    push %a
    mov %a, %c
    mov %c, 0
    lw %b, [PSR3]
    add %a, %b
    adc %c
    lw %b, [PSR5]
    add %a, %b
    adc %c
    lw %b, [PSR6]
    add %a, %b
    adc %c

    mov %d, %c
    lw %c, [PSR7]
    add %d, %c
    mov %c, %a
    pop %b
    lw %a, [PSR0]
    ret
