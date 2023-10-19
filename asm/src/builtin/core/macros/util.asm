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
        inc %l, %h
        sw $vh
    }
    ($to: expr, $vl: reg, $vh: reg) => {
        ldhl $to
        sw $vl, $vh
    }
    ($to: expr, $b: expr) => {
        mov %f, $b.l
        sw $to, %f
    }
}
  
#[macro] lw: {
    ($tol: reg, $toh: reg) => {
        lw $tol
        inc %l, %h
        lw $toh
    }
    ($tol: reg, $toh: reg, $addr: expr) => {
        ldhl $addr
        lw $tol, $toh
    }
    ($tol: reg, $toh: reg, $addrl: lit | reg, $addrh: lit | reg) => {
        mov %l, $addrl 
        mov %h, $addrh
        lw $tol, $toh
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
