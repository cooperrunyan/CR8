#[use(std)]

#[boot]
main:
    mov %a, 2
    mov %b, 2
    mov %c, 2
    mov %d, 2
    call [lsa]
    dbg
    halt
