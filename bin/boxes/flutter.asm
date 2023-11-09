#[use(std::gfx::grid)]

#[dyn(COUNTER: 2)]
#[static(COUNTER_VAL: 0x0f00)]


#[main]
main:
    bank 1

    call init_counter

    .loop:
        in %a, RNG
        in %b, RNG
        call filled_box

        lw %a, COUNTER
        lw %b, COUNTER + 1
        dec %a, %b
        sw COUNTER, %a
        sw COUNTER + 1, %b
        jnz .loop, %a, %b

        ; Re-init the counter and fall through to [.loop_clear]
        call init_counter
        jmp .loop_clear


    .loop_clear:
        in %a, RNG
        in %b, RNG
        call clear_box

        lw %a, COUNTER
        lw %b, COUNTER + 1
        dec %a, %b
        sw COUNTER, %a
        sw COUNTER + 1, %b
        jnz .loop_clear, %a, %b

        ; Re-init the counter and go back to [.loop]
        call init_counter

        jmp .loop

init_counter:
    mov %a, COUNTER_VAL & 0xFF
    mov %b, COUNTER_VAL >> 8
    sw COUNTER, %a
    sw COUNTER + 1, %b
    ret
