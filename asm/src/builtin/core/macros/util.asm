#[macro] mov: {
    ($inlo: reg, $inhi: reg, $frlo: any, $frhi: any) => {
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
    ($a: reg, $b: reg) => {
        sw $a
        inc %x, %y
        sw $b
    }
}

#[macro] lw: {
    ($a: reg, $b: reg) => {
        lw $a
        inc %x, %y
        lw $b
    }
}

; do nothing for 1 tick
#[macro] nop: {
    () => {
      mov %a, %a ; 1 byte
    }
}

#[macro] push: {
    ($l: any, $h: any) => {
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
