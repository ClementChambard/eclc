use super::*;

use magic_unwrapper::EnumUnwrap;

#[derive(Debug, Clone, EnumUnwrap)]
pub enum Param {
    Int(String),
    Float(String),
}

#[derive(Debug, Clone)]
pub struct Sub {
    pub name: String,
    pub params: Vec<Param>,
    pub instructions: Vec<Instr>,
}

impl Sub {
    fn replace_vars(&mut self) {
        let mut scope = variables::Scope::new();
        for p in &self.params {
            match p {
                Param::Int(name) => scope.add_var(name, 1),
                Param::Float(name) => scope.add_var(name, 2),
            }
        }

        self.instructions = variables::replace_in_bloc(&mut scope, &self.instructions);

        if scope.max_offset > 0 {
            self.instructions.insert(
                0,
                Instr::Call("ins_40".to_string(), vec![Expr::Int(scope.max_offset)]),
            );
        }
    }

    pub fn gen_label(&self, lbl_seed: &mut usize) -> String {
        let mut n = self.name.clone();
        n.push_str(&format!("_label_{}", lbl_seed));
        *lbl_seed += 1;
        n
    }

    fn flatten_bloc(bloc: &Vec<Instr>) -> Vec<Instr> {
        let mut new_instructions = Vec::new();
        for i in bloc {
            match i {
                Instr::Bloc(l) => new_instructions.extend(Self::flatten_bloc(l)),
                _ => new_instructions.push(i.clone()),
            }
        }
        new_instructions
    }

    fn resolve_push_expr(&mut self) {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => new_instructions.extend(e.instructions()),
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
    }

    fn check_expressions(&mut self) {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => {
                    let mut e = e.clone();
                    e.anotate();
                    e.constant_fold();
                    new_instructions.push(Instr::PushExpr(e));
                }
                Instr::Call(name, v) => {
                    let mut args = Vec::new();
                    let mut stoff = -1;
                    for e in v {
                        let mut e = e.clone();
                        e.anotate();
                        e.constant_fold();
                        if e.is_primitive() {
                            args.push(e);
                        } else {
                            let t = e.get_type();
                            new_instructions.push(Instr::PushExpr(e));
                            match t {
                                ExprType::Int => args.push(Expr::VarInt(stoff)),
                                ExprType::Float => args.push(Expr::VarFloat(stoff as f32)),
                                _ => panic!("Can't push non number onto the stack"),
                            }
                            stoff -= 1;
                        }
                    }
                    let ins_opcode = crate::ecl_instructions::matching_ins_sep(name, &args);
                    let new_name = String::from(format!("ins_{ins_opcode}"));
                    // if vararg, insert type markers
                    new_instructions.push(Instr::Call(new_name, args));
                }
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
    }

    fn check_if_sub_returns(&mut self) {
        // CAREFUL: a jump instruction could jump over a return ?
        let last = self.instructions.iter().last();
        if let Some(last) = last {
            match last {
                Instr::Call(name, _) => {
                    let opcode = crate::code_gen::resolve_ins_opcode(&name);
                    if opcode != 10 && opcode != 1 {
                        self.instructions
                            .push(Instr::Call("ins_10".to_string(), vec![]));
                    }
                }
                _ => self
                    .instructions
                    .push(Instr::Call("ins_10".to_string(), vec![])),
            }
        } else {
            self.instructions
                .push(Instr::Call("ins_10".to_string(), vec![]));
        }
    }

    pub fn process(&mut self) {
        let mut lbl_seed = 0usize;
        self.replace_vars();
        self.instructions = builtin_idents::replace(&self.instructions);
        self.instructions = if_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        self.instructions = loop_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        self.instructions = while_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        // desugar other
        // maybe resolve variables before flattening anything.
        self.instructions = Self::flatten_bloc(&self.instructions);
        self.check_if_sub_returns();
        self.check_expressions();
        self.resolve_push_expr();
        self.resolve_labels();
        // optimize jump chain and remove dead code at some point
        // resolve other identifiers: vars, constants ... (right now there is none)
    }

    fn resolve_labels(&mut self) {
        let mut labels = std::collections::HashMap::new();
        let mut new_instructions = vec![];
        let mut pos = 0;
        for i in &self.instructions {
            match i {
                Instr::Label(lbl) => {
                    labels.insert(lbl.clone(), Expr::Int(pos as i32));
                }
                _ => new_instructions.push(i.clone()),
            }
            pos += i.size();
        }
        for ni in &mut new_instructions {
            match ni {
                Instr::Call(_, v) => {
                    for e in v.iter_mut() {
                        e.replace_all_id(&labels);
                    }
                }
                _ => {}
            }
        }
        self.instructions = new_instructions;
    }
}

fn resolve_param(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1);
    assert!(args.len() == 1);
    let typ = &typ[0];
    AstNode::Param(match &typ[..] {
        "Int" => Param::Int(args[0].clone().token().id()),
        "Float" => Param::Float(args[0].clone().token().id()),
        _ => panic!("Unknown param type"),
    })
}

fn resolve_sub(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.is_empty());
    assert!(args.len() == 3);
    let val = args[0].clone().token().id();
    let param_list = args[1].clone().list();
    let ins_list = args[2].clone().list();
    AstNode::Sub(Sub {
        name: val.clone(),
        params: param_list.into_iter().map(|n| n.param()).collect(),
        instructions: ins_list.into_iter().map(|n| n.instr()).collect(),
    })
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Sub", resolve_sub);
    resolver.add_func("Param", resolve_param);
}
