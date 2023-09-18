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
        send &CTRL, &CTRLHALT
    }
}

#macro ping {
    () => {
        send &CTRL, &CTRLPING
    }
}

#macro dbg {
    () => {
        send &CTRL, &CTRLDBG
    }
}

#macro peek {
    ($addr: imm16) => {
        send &CTRL, &CTRLPEEK
        send &CTRL, $addr.l
        send &CTRL, $addr.h
    }
}

