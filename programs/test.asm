#[use(std)]

#[boot]
main:
    mov %a, 20
    mov %b, 15
    mov %c, 94
    mov %d, 87
    brkpt
    call [mul16]
    brkpt
    ping
    halt
