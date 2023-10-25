#![micro]

mov: {
    (R2I0) => {
        ; the 'lr' read signal puts high nibble into
        ; rhs and low nibble into lhs
        aw pc, dw mem, dr lr
        dw sel, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lr, pc++
        aw pc, dw mem, dr rhs
        dw rhs, dr sel, pc++
    }
}

jnz: {
    (R1I0) => {
        aw pc, dw mem, dr lr, pc++
        alu cmp, dw alflg, dr f
        aw xy, pc jnz
    }
    (R0I1) => {
        aw xy, pc j
    }
}

lw: {
    ; LW imm16
    (R1I1) => {
        aw pc, dw mem, dr io, pc++
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        aw lr, dw mem, dr rhs
        dw io, dr lhs
        dw rhs, dr sel, pc++
    }
    ; LW XY
    (R1I0) => {
        aw pc, dw mem, dr lhs
        aw xy, dw mem, dr sel, pc++
    }
}

sw: {
    (R1I0) => {
        aw pc, dw mem, dr rhs, pc++
        aw xy, dw sel, dr mem
    }
    (R1I1) => {
        aw pc, dw mem, dr rhs, pc++
        dw sel, dr io
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        aw lr, dw io, dr mem, pc++
    }
}

push: {
    (R1I0) => {
        aw pc, dw mem, dr rhs, pc++
        aw sp, dw sel, dr mem, sp++
    }
    (R0I1) => {
        aw pc, dw mem, dr rhs, pc++
        aw sp, dw rhs, dr mem, sp++
    }
}

pop: {
    (R1I0) => {
        aw pc, dw mem, dr lhs, pc++, sp--
        aw sp, dw mem, dr sel
    }
}

in: {}

out: {}

adc: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu add, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu add, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

sbb: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

cmp: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu cmp, dw alflg, dr f, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu cmp, dw alflg, dr f, pc++
    }
}

and: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

or: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

nor: {
    (R2I0) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (R1I1) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

