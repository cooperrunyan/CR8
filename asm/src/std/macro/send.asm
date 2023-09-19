#[use(std::arch)]

#[macro] send: {
    ($port: imm8, $b: imm8) => {
        mov %f, $b
        out $port, %f
    }
}

#[macro] halt: {
    () => {
        send [CTRL], [SIGHALT]
    }
}

#[macro] ping: {
    () => {
        send [CTRL], [SIGPING]
    }
}

#[macro] brkpt: {
    () => {
        send [CTRL], [SIGBRKPT]
    }
}

#[macro] dbg: {
    () => {
        send [CTRL], [SIGDBG]
    }
}

