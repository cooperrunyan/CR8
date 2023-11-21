#![micro]

; Note: every instruction starts with either "dr lhs" or "dr rhs".
; this allows us to set the two flags between the fetch and decode
; stages, saving a clock cycle.

; SIDE EFFECTS: first signal is excluded from the microcode rom.
; if an instruction needs to do anything other than reading lhs or
; rhs, it can have a "nop" to opt out of the preloading. Also, logic
; for the initial lhs/rhs is found in the circuit, not the microcode.

mov: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs
        dw sel, dr sel, pc inc
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr sel, pc inc
    }
}

jnz: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu cmp, dw alflg, dr f
        aw xy, pc jnz
    }
    (imm) => {
        dr lhs

        alu cmp, dw alflg, dr f
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, pc jnz
    }
}

jmp: {
    (reg) => {
        nop

        aw xy, pc jmp
    }
    (imm) => {
        nop
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, pc jmp
    }
}

lw: {
    (imm) => {
        nop

        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs, pc inc
        aw lr, dw mem, dr rhs
        dw op, dr lhs
        dw rhs, dr sel
    }
    (reg) => {
        dr lhs

        aw xy, dw mem, dr sel, pc inc
    }
}

sw: {
    (reg) => {
        dr rhs

        aw xy, dw sel, dr mem
    }
    (imm) => {
        dr rhs

        dw sel, dr io
        aw pc, dw mem, dr lhs, pc inc
        aw pc, dw mem, dr rhs
        aw lr, dw io, dr mem, pc inc
    }
}

push: {
    (reg) => {
        dr rhs

        aw sp, dw sel, dr mem, sp inc
    }
    (imm) => {
        dr rhs

        aw sp, dw rhs, dr mem, sp inc
    }
}

pop: {
    (reg) => {
        dr lhs

        sp dec
        aw sp, dw mem, dr sel
    }
}

in: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr io
        dw dev, dr sel
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr io, pc inc
        dw dev, dr sel
    }
}

out: {
    (reg) => {
        dr rhs

        pc inc, dw sel, dr io
        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr dev
    }
    (imm) => {
        dr rhs

        dw sel, dr io, pc inc
        aw pc, dw mem, dr dev, pc inc
    }
}

adc: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu adc, dw alu, dr io
        alu adc, dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu adc, dw alu, dr io
        alu adc, dw alflg, dr f
        dw io, dr sel
    }
}

sbb: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu sbb, dw alu, dr io
        alu sbb, dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu sbb, dw alu, dr io
        alu sbb, dw alflg, dr f
        dw io, dr sel
    }
}

cmp: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu cmp, dw alflg, dr f
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu cmp, dw alflg, dr f
    }
}

and: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu and, dw alu, dr io
        alu and, dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu and, dw alu, dr io
        alu and, dw alflg, dr f
        dw io, dr sel
    }
}

or: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu or, dw alu, dr io
        alu or, dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        alu or, dw alu, dr io
        alu or, dw alflg, dr f
        dw io, dr sel
    }
}

nor: {
    (reg) => {
        dr lhs

        aw pc, dw mem, dr rhs, pc inc
        dw sel, dr rhs
        alu nor, dw alu, dr io
        alu nor, dw alflg, dr f
        dw io, dr sel
    }
    (imm) => {
        dr lhs

        alu nor, dw alu, dr io
        alu nor, dw alflg, dr f
        dw io, dr sel
    }
}

bank: {
    (reg) => {
        dr rhs

        dr k, dw sel, pc inc
    }
    (imm) => {
        nop

        aw pc, dw mem, dr k, pc inc
    }
}
