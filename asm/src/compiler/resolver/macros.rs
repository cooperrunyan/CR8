use std::collections::HashMap;

use crate::compiler::ast::{AddrByte, AstNode, Ident, Instruction, MacroArg, ToNode, Value};

use super::Compiler;

impl Compiler {
    pub(crate) fn resolve_macros(&mut self) {
        let mut new_tree = vec![];

        let mut tree = vec![];
        tree.append(&mut self.tree);

        for node in tree {
            let mut stripped = self.fill_macro(node);
            new_tree.append(&mut stripped);
        }

        self.tree = new_tree;
    }

    fn fill_macro(&self, node: AstNode) -> Vec<AstNode> {
        let mut tree = vec![];

        match node {
            AstNode::Instruction(Instruction::Macro(mac_name, mut args)) => {
                args.reverse();
                let mac_content = match self.macros.get(&mac_name) {
                    Some(m) => m,
                    None => panic!("Macro '{mac_name}' not defined"),
                };

                let mut parsed_args: HashMap<String, Value> = HashMap::new();

                for (i, mac_arg) in mac_content.args.iter().enumerate() {
                    match mac_arg {
                        MacroArg::Immediate(name) => {
                            let Some(next) = args.pop() else {
                                panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected immediate, found none.");
                            };
                            match next {
                            Value::Immediate(v) => parsed_args.insert(name.to_string(),  Value::Immediate(v)),
                            Value::Ident(id) => parsed_args.insert(name.to_string(), Value::Ident(id)),
                            _ => panic!("Expected an immediate value at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                        }
                        MacroArg::Register(name) => {
                            let Some(next) = args.pop() else {
                                panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected register, found none.");
                            };
                            match next {
                            Value::Register(r) => parsed_args.insert(name.to_string(),  Value::Register(r)),
                            _ => panic!("Expected a register at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                        }
                        MacroArg::ImmReg(name) => {
                            let Some(next) = args.pop() else {
                                panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected Immediate or Register, found none.");
                            };
                            match next {
                            Value::Immediate(v) => parsed_args.insert(name.to_string(),  Value::Immediate(v)),
                            Value::Register(r) => parsed_args.insert(name.to_string(),  Value::Register(r)),
                            Value::Ident(id) => parsed_args.insert(name.to_string(), Value::Ident(id)),
                            _ => panic!("Expected an immediate or register at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                        }
                        MacroArg::Addr(name) => {
                            let Some(next) = args.pop() else {
                                panic!("Bad amount of arguments sent to macro: '{mac_name:#?}'. Expected an address, found none.");
                            };
                            match next {
                            Value::Ident(Ident::Addr(a)) => {
                                parsed_args.insert(format!("{name}"), Value::Ident(Ident::Addr(a.clone())));
                                parsed_args.insert(format!("{name}l"), Value::AddrByte(AddrByte::Low(a.clone())));
                                parsed_args.insert(format!("{name}h"), Value::AddrByte(AddrByte::High(a)));
                            },
                            Value::Expression(expr) => {
                                parsed_args.insert(format!("{name}"), Value::Expression(expr.clone()));
                                parsed_args.insert(format!("{name}l"), Value::AddrByte(AddrByte::Low(expr.clone())));
                                parsed_args.insert(format!("{name}h"), Value::AddrByte(AddrByte::High(expr)));
                            },
                            _ => panic!("Expected an address at {mac_name:#?} argument {i}. Received: {next:#?}")
                        };
                        }
                    }
                }
                for instruction in mac_content.body.iter() {
                    let (empty, args) = match instruction {
                        Instruction::Macro(m, args) => {
                            (Instruction::Macro(m.to_string(), vec![]), args)
                        }
                        Instruction::Native(n, args) => {
                            (Instruction::Native(n.to_owned(), vec![]), args)
                        }
                    };
                    let mut new_args: Vec<Value> = vec![];

                    for arg in args {
                        match arg {
                            Value::Ident(Ident::MacroArg(ma)) => {
                                let Some(val) = parsed_args.get(ma) else {
                                    panic!("Attempted to use undefined macro arg at {mac_name:#?} {empty:#?}");
                                };
                                new_args.push(val.clone().to_owned());
                            }
                            oth => new_args.push(oth.clone()),
                        }
                    }

                    match instruction {
                        Instruction::Macro(m, _) => {
                            let mut nodes = self
                                .fill_macro(Instruction::Macro(m.to_owned(), new_args).to_node());
                            tree.append(&mut nodes);
                        }
                        Instruction::Native(n, _) => {
                            tree.push(Instruction::Native(n.to_owned(), new_args).to_node())
                        }
                    };
                }
            }
            _ => tree.push(node),
        };

        tree
    }
}
