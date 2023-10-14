#[use(std)]
#[use("./lib/box")]

#[dyn(COUNTER: 2)]
#[static(COUNTER_VAL: 10)]

#[boot]
main:
    mov %mb, 1

    ; Keep a counter on the stack
    call [init_counter]

    .loop:
        in %a, [RNG]
        in %b, [RNG]
        call [box]

        lw %a, %b, [COUNTER]
        dec %a, %b
        sw [COUNTER], %a, %b
        jnz [.loop], %a, %b

        ; Re-init the counter and fall through to [.loop_clear]
        call [init_counter]
        jmp [.loop_clear]


    .loop_clear:
        in %a, [RNG]
        in %b, [RNG]
        call [clear_box]

        lw %a, %b, [COUNTER]
        dec %a, %b
        sw [COUNTER], %a, %b
        jnz [.loop_clear], %a, %b

        ; Re-init the counter and go back to [.loop]
        call [init_counter]

        jmp [.loop]

init_counter:
    mov %a, [COUNTER_VAL & 0xFF]
    mov %b, [COUNTER_VAL >> 8]
    sw [COUNTER], %a, %b
    ret
