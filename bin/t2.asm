#[use(std)]

#[main]
main:
  mov %c, 25

  .loop:
    dec %c
    jnz .loop, %c
    dbg
    halt
