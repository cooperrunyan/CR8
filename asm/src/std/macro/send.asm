#[use(std::arch)]

#[macro] send: {
    ($port: imm8, $b: imm8) => {
        mov %f, $b
        out $port, %f
    }
}

#[macro] halt: {
    () => {
        send [CTRL], [HALT]
    }
}

#[macro] ping: {
    () => {
        send [CTRL], [PING]
    }
}

#[macro] dbg: {
    () => {
        send [CTRL], [DBG]
    }
}

#[macro] peek: {
    ($l: imm8, $h: imm8) => {
        send [CTRL], [PEEK]
        send [CTRL], $l
        send [CTRL], $h
    }
    ($addr: imm16) => {
        peek $addr.l, $addr.h
    }
    ($l: reg, $h: reg) => {
        send [CTRL], [PEEK]
        out [CTRL], $l
        out [CTRL], $h
    }
}

