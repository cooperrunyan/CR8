#[macro] mov: {
    ($inlo: reg, $inhi: reg, $frlo: reg | imm8, $frhi: reg | imm8) => {
        mov $inlo, $frlo
        mov $inhi, $frhi
    }
    ($inlo: reg, $inhi: reg, $from: imm16) => {
        mov $inlo, $from.l
        mov $inhi, $from.h
    }
}

#[macro] sw: {
    ($to: imm16, $b: imm8) => {
        mov %f, $b
        sw $to, %f
    }
    ($b: imm8) => {
        mov %f, $b
        sw %f
    }
    ($vl: reg, $vh: reg) => {
        sw $vl
        inc %l, %h
        sw $vh
    }
    ($to: imm16, $vl: reg, $vh: reg) => {
        ldhl $to
        sw $vl, $vh
    }
}

#[macro] lw: {
    ($tol: reg, $toh: reg) => {
        lw $tol
        inc %l, %h
        lw $toh
    }
    ($tol: reg, $toh: reg, $addr: imm16) => {
        ldhl $addr
        lw $tol, $toh
    }
    ($tol: reg, $toh: reg, $addrl: imm8 | reg, $addrh: imm8 | reg) => {
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
    ($l: imm8 | reg, $h: imm8 | reg) => {
        push $l
        push $h
    }
}

#[macro] pushx: {
    ($a: imm16) => {
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
