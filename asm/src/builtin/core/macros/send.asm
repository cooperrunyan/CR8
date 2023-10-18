#[use(core::sys)]

#[macro] send: {
    ($port: lit, $b: lit) => {
        mov %f, $b
        out $port, %f
    }
    ($port: expr, $b: expr) => {
        mov %f, $b.l
        out $port, %f
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
        send CTRL, SIGDBG
    }
}

