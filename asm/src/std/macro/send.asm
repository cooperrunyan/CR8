#include "<std>/arch"

;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/send

#macro send {
    ($port: imm8, $b: imm8) => {
        mov %f, $b
        out $port, %f
    }
}

#macro halt {
    () => {
        send [CTRL], [CTRLHALT]
    }
}

#macro ping {
    () => {
        send [CTRL], [CTRLPING]
    }
}

#macro dbg {
    () => {
        send [CTRL], [CTRLDBG]
    }
}

#macro peek {
    ($l: imm8, $h: imm8) => {
        send [CTRL], [CTRLPEEK]
        send [CTRL], $l
        send [CTRL], $h
    }
    ($addr: imm16) => {
        peek $addr.l, $addr.h
    }
    ($l: reg, $h: reg) => {
        send [CTRL], [CTRLPEEK]
        out [CTRL], $l
        out [CTRL], $h
    }
}

