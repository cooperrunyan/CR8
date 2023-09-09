use std::collections::HashMap;

use crate::ast::{AddrByte, Ast, AstNode, Expression, Ident, Instruction, Label, Value};
use crate::op::Operation;

#[derive(Debug, Default)]
pub struct Compiler {
    pub bin: Vec<u8>,
    ast: Ast,
    labels: HashMap<String, usize>,
    last_label: String,
    pc: usize,
}

impl From<Ast> for Compiler {
    fn from(ast: Ast) -> Self {
        Self {
            ast,
            ..Default::default()
        }
    }
}

impl Compiler {
    pub fn strip_labels(&mut self) {
        for node in self.ast.tree.iter() {
            match node {
                AstNode::Label(Label::Label(ln)) => {
                    self.last_label = ln.to_string();
                    self.labels.insert(ln.to_owned(), self.pc);
                }
                AstNode::Label(Label::SubLabel(sub)) => {
                    self.labels
                        .insert(format!("{}{sub}", self.last_label), self.pc);
                }
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let size = match op.size(args) {
                        Ok(sz) => sz,
                        Err(e) => panic!("Argument error for {op:#?}: {e:#?}. At {node:#?}"),
                    };
                    self.pc += size as usize;
                }
                oth => panic!("Unexpected {oth:#?}. At {node:#?}"),
            }
        }
        dbg!(&self.labels);
    }

    fn resolve_addr(&mut self, name: &str) -> Result<i128, ()> {
        if let Some(stat) = self.ast.statics.get(name) {
            Ok(stat.to_owned() as i128)
        } else if let Some(label) = self.labels.get(name) {
            Ok(label.to_owned() as i128)
        } else if let Some(label) = self.labels.get(&format!("{}{}", self.last_label, name)) {
            Ok(label.to_owned() as i128)
        } else if let Some(ram_loc) = self.ast.ram_locations.get(name) {
            Ok((*ram_loc as i128) + self.ast.ram_origin as i128)
        } else {
            Err(())
        }
    }

    fn resolve_static(&mut self, name: &str) -> Result<i128, ()> {
        let Some(stat) = self.ast.statics.get(name) else {
            return Err(());
        };
        return Ok(stat.to_owned() as i128);
    }

    fn resolve_ident(&mut self, ident: &Ident) -> Result<i128, ()> {
        match ident {
            Ident::Addr(a) => self.resolve_addr(&a),
            Ident::Static(s) => self.resolve_static(&s),
            Ident::PC => Ok(self.pc as i128),
            _ => Err(()),
        }
    }

    fn resolve_expr(&mut self, expr: &Expression) -> Result<i128, ()> {
        Err(())
    }

    pub fn compile(&mut self) {
        use Operation::*;

        self.last_label = String::new();

        let mut tree = vec![];
        tree.append(&mut self.ast.tree);
        self.ast.tree = vec![];

        for node in tree {
            match node {
                AstNode::Label(Label::Label(ln)) => self.last_label = ln,
                AstNode::Instruction(Instruction::Native(op, args)) => {
                    let mut header = (op as u8) << 4;
                    let mut compiled_args: Vec<u8> = vec![];
                    let mut regn = 0;

                    for arg in args {
                        match arg {
                            Value::AddrByte(AddrByte::High(a)) => match self.resolve_addr(&a) {
                                Ok(a) => compiled_args.push((a >> 8) as u8),
                                Err(()) => panic!("Unknown address: {a:#?}"),
                            },
                            Value::AddrByte(AddrByte::Low(a)) => match self.resolve_addr(&a) {
                                Ok(a) => compiled_args.push(a as u8),
                                Err(()) => panic!("Unknown address: {a:#?}"),
                            },
                            Value::Register(r) => {
                                if regn > 0 {
                                    compiled_args.push(r as u8)
                                } else {
                                    header |= r as u8
                                }
                                regn += 1;
                            }
                            Value::Immediate(imm) => compiled_args.push(imm as u8),
                            Value::Ident(id) => match self.resolve_ident(&id) {
                                Ok(a) => {
                                    compiled_args.push(a as u8);
                                    if op == LW || op == SW {
                                        compiled_args.push((a >> 8) as u8);
                                    }
                                }
                                Err(()) => panic!("Unknown identifier: {id:#?}"),
                            },
                            Value::Expression(exp) => match self.resolve_expr(&exp) {
                                Err(()) => panic!("Failed to resolve: {exp:#?}"),
                                Ok(v) => {
                                    compiled_args.push(v as u8);
                                    compiled_args.push((v >> 8) as u8);
                                }
                            },
                        }
                    }
                    match (op, regn, compiled_args.len()) {
                        (LW, 1, 0) => header |= 0b00001000,
                        (SW, 1, 2) => header |= 0b00001000,
                        (PUSH | JNZ, 0, 1) => header |= 0b00001000,
                        (MOV | OUT | IN | CMP | ADC | SBB | OR | NOR | AND, 1, 1) => {
                            header |= 0b00001000
                        }
                        _ => {}
                    };
                    self.bin.push(header);
                    self.bin.append(&mut compiled_args);
                }
                _ => {}
            }
        }
    }
}

// (LW, true) => self.lw_imm16(Register::try_from(reg_bits)?, (b0, b1)),
// (LW, false) => self.lw_hl(Register::try_from(reg_bits)?),
// (SW, true) => self.sw_imm16((b0, b1), Register::try_from(reg_bits)?),
// (SW, false) => self.sw_hl(Register::try_from(reg_bits)?),
// (MOV, true) => self.mov_imm8(Register::try_from(reg_bits)?, b0),
// (MOV, false) => {
//     self.mov_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (PUSH, true) => self.push_imm8(b0),
// (PUSH, false) => self.push_reg(Register::try_from(reg_bits)?),
// (POP, _) => self.pop(Register::try_from(reg_bits)?),
// (JNZ, true) => self.jnz_imm8(b0),
// (JNZ, false) => self.jnz_reg(Register::try_from(reg_bits)?),
// (IN, true) => self.in_imm8(Register::try_from(reg_bits)?, b0),
// (IN, false) => self.in_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?),
// (OUT, true) => self.out_imm8(b0, Register::try_from(reg_bits)?),
// (OUT, false) => {
//     self.out_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (CMP, true) => self.cmp_imm8(Register::try_from(reg_bits)?, b0),
// (CMP, false) => {
//     self.cmp_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (ADC, true) => self.adc_imm8(Register::try_from(reg_bits)?, b0),
// (ADC, false) => {
//     self.adc_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (SBB, true) => self.sbb_imm8(Register::try_from(reg_bits)?, b0),
// (SBB, false) => {
//     self.sbb_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (OR, true) => self.or_imm8(Register::try_from(reg_bits)?, b0),
// (OR, false) => self.or_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?),
// (NOR, true) => self.nor_imm8(Register::try_from(reg_bits)?, b0),
// (NOR, false) => {
//     self.nor_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
// (AND, true) => self.and_imm8(Register::try_from(reg_bits)?, b0),
// (AND, false) => {
//     self.and_reg(Register::try_from(reg_bits)?, Register::try_from(b0)?)
// }
