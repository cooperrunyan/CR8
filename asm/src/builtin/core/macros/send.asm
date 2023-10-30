#[use(core::sys)]

#[macro] send: {
    ($port: expr, $b: expr) => {
        mov %f, $b.l
        out $port.l, %f
    }
}

#[macro] halt: {
    () => {
        send CTRL, SIGHALT
    }
}

#[macro] ping: {
    () => {
        send CTRL, SIGPING
    }
}

#[macro] brkpt: {
    () => {
        send CTRL, SIGBRKPT
    }
}

#[macro] dbg: {
    () => {
        push %f
        send CTRL, SIGDBG
        pop %f
        out 0x00, %a
        out 0x00, %b
        out 0x00, %c
        out 0x00, %d
        out 0x00, %z
        out 0x00, %f
        out 0x00, %x
        out 0x00, %y
    }
}
