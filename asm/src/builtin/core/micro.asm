#![micro]

mov: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs
        dw sel, dr sel, pc inc
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr sel, pc inc
    }
}

jnz: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu cmp, dw alflg, dr f
        aw xy, pc jnz
    }
    (imm) => {
        dw op, dr lhs
        alu cmp, dw alflg, dr f
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, pc jnz
    }
}

jmp: {
    (reg) => {
        aw xy, pc jmp
    }
    (imm) => {
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, pc jmp
    }
}

lw: {
    (imm) => {
        dw op, dr io ; use io as intermediate
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, dw mem, dr rhs
        dw io, dr lhs
        dw rhs, dr sel
    }
    (reg) => {
        dw op, dr io
        aw xy, dw mem, dr sel, pc inc
    }
}

sw: {
    (reg) => {
        dw op, dr rhs
        aw xy, dw sel, dr mem
    }
    (imm) => {
        dw op, dr rhs
        dw sel, dr io
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs
        aw lr, dw io, dr mem, pc inc
    }
}

push: {
    (reg) => {
        dw op, dr rhs
        aw sp, dw sel, dr mem, sp inc
    }
    (imm) => {
        dw op, dr rhs
        aw sp, dw rhs, dr mem, sp inc
    }
}

pop: {
    (reg) => {
        dw op, dr lhs, sp dec
        aw sp, dw mem, dr sel
    }
}

in: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr io
        dw dev, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr io, pc inc
        dw dev, dr sel
    }
}

out: {
    (reg) => {
        dw op, dr rhs, pc inc
        dw sel, dr io
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr dev
    }
    (imm) => {
        dw op, dr rhs, pc inc
        dw sel, dr io
        aw pc, dw mem, dr dev, pc inc
    }
}

adc: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu add, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu add, dw alu, dr io
        dw alflg, dr f
    }
}

sbb: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu sub, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
}

cmp: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu cmp, dw alflg, dr f
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu cmp, dw alflg, dr f
    }
}

and: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu and, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
}

or: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu or, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
}

nor: {
    (reg) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dw op, dr lhs
        aw pc, dw mem, dr rhs, pc inc
        alu nor, dw alu, dr io
        dw alflg, dr f
        dw io, dr sel
    }
}

bank: {
    (reg) => {
        dw op, dr rhs, pc inc
        dr k, dw sel
    }
    (imm) => {
        aw pc, dw mem, dr k, pc inc
    }
}
