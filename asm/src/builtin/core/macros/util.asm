#[macro] mov: {
    ($inlo: reg, $inhi: reg, $frlo: reg | lit, $frhi: reg | lit) => {
        mov $inlo, $frlo
        mov $inhi, $frhi
    }
    ($inlo: reg, $inhi: reg, $from: expr) => {
        mov $inlo, $from.l
        mov $inhi, $from.h
    }
}

#[macro] sw: {
    ($to: expr, $b: lit) => {
        mov %f, $b
        sw $to, %f
    }
    ($b: lit) => {
        mov %f, $b
        sw %f
    }
    ($vl: reg, $vh: reg) => {
        sw $vl
        inc %x, %y
        sw $vh
    }
    ($tol: reg, $toh: reg, $r: reg) => {
        ldxy $tol, $toh
        sw $r
    }
    ($to: expr, $vl: reg, $vh: reg) => {
        ldxy $to
        sw $vl, $vh
    }
}

#[macro] lw: {
    ($tol: reg, $toh: reg) => {
        lw $tol
        inc %x, %y
        lw $toh
    }
    ($tol: reg, $toh: reg, $addr: expr) => {
        ldxy $addr
        lw $tol, $toh
    }
    ($tol: reg, $toh: reg, $addrl: lit | reg, $addrh: lit | reg) => {
        mov %x, $addrl
        mov %y, $addrh
        lw $tol, $toh
    }
    ($to: reg, $addrl: reg, $addrh: reg) => {
        ldxy $addrl, $addrh
        lw $to
    }
}

; do nothing for 2 ticks
#[macro] nop: {
    () => {
      mov %a, %a ; 2 bytes
    }
}

#[macro] push: {
    ($l: lit | reg, $h: lit | reg) => {
        push $l
        push $h
    }
}

#[macro] pushx: {
    ($a: expr) => {
        push $a.l
        push $a.h
    }
}

#[macro] pop: {
    ($l: reg, $h:  reg) => {
        pop $h
        pop $l
    }
}
