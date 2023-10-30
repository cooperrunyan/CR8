#![micro]

mov: {
    (reg) => {
        ; the 'lr' read signal puts high nibble into
        ; rhs and low nibble into lhs
        aw pc, dw mem, dr lr
        dw sel, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lr, pc++
        aw pc, dw mem, dr rhs
        dw rhs, dr sel, pc++
    }
}

jnz: {
    (reg) => {
        aw pc, dw mem, dr lr, pc++
        alu cmp, dw alflg, dr f
        aw xy, pc jnz
    }
    (imm) => {
        aw xy, pc j
    }
}

lw: {
    ; LW imm16
    (imm) => {
        aw pc, dw mem, dr io, pc++
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        aw lr, dw mem, dr rhs
        dw io, dr lhs
        dw rhs, dr sel, pc++
    }
    ; LW XY
    (reg) => {
        aw pc, dw mem, dr lhs
        aw xy, dw mem, dr sel, pc++
    }
}

sw: {
    (reg) => {
        aw pc, dw mem, dr rhs, pc++
        aw xy, dw sel, dr mem
    }
    (imm) => {
        aw pc, dw mem, dr rhs, pc++
        dw sel, dr io
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        aw lr, dw io, dr mem, pc++
    }
}

push: {
    (reg) => {
        aw pc, dw mem, dr rhs, pc++
        aw sp, dw sel, dr mem, sp++
    }
    (imm) => {
        aw pc, dw mem, dr rhs, pc++
        aw sp, dw rhs, dr mem, sp++
    }
}

pop: {
    (reg) => {
        aw pc, dw mem, dr lhs, pc++, sp--
        aw sp, dw mem, dr sel
    }
}

in: {}

out: {}

adc: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu add, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu add, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

sbb: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

cmp: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu cmp, dw alflg, dr f, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu cmp, dw alflg, dr f, pc++
    }
}

and: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

or: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

nor: {
    (reg) => {
        aw pc, dw mem, dr lr
        dw sel, dr rhs
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc++
        aw pc, dw mem, dr rhs
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel, pc++
    }
}

