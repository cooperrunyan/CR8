#[use(std)]

#[boot]
main:
    mov %a, 4
    mov %b, 0
    mov %c, 0
    mov %d, 1
    call mul16
    halt
