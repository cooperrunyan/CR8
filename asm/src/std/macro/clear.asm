;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
; <std>/macro/clear

#macro clrf {
    () => {
        mov %f, 0
    }
}

#macro clrfb {
    () => {
        and %f, 0b0111
    }
}

#macro clrfc {
    () => {
        and %f, 0b1011
    }
}
