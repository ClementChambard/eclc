use crate::{
    ecl_instructions::{MatchInsResult, MatchType},
    error::Error,
};

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
    fn replace_vars(&mut self) -> Result<(), Error> {
        let mut scope = variables::Scope::new();
        for p in &self.params {
            match p {
                Param::Int(name) => scope.add_var(name, 1)?,
                Param::Float(name) => scope.add_var(name, 2)?,
            }
        }

        self.instructions = variables::replace_in_bloc(&mut scope, &self.instructions)?;

        if scope.max_offset > 0 {
            self.instructions.insert(
                0,
                Instr::Call("ins_40".to_string(), vec![Expr::Int(scope.max_offset)]),
            );
        }
        Ok(())
    }

    pub fn gen_label(&self, lbl_seed: &mut usize) -> String {
        let mut n = self.name.clone();
        n.push_str(&format!("_label_{}", lbl_seed));
        *lbl_seed += 1;
        n
    }

    fn resolve_push_expr(&mut self) -> Result<(), Error> {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => new_instructions.extend(e.instructions()?),
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
        Ok(())
    }

    fn check_expressions(&mut self) -> Result<(), Error> {
        let mut new_instructions = Vec::new();
        for i in &self.instructions {
            match i {
                Instr::PushExpr(e) => {
                    let mut e = e.clone();
                    e.anotate()?;
                    e.constant_fold();
                    new_instructions.push(Instr::PushExpr(e));
                }
                Instr::Call(name, v) => {
                    let mut args = Vec::new();
                    let mut stoff = -1;
                    for e in v {
                        let mut e = e.clone();
                        e.anotate()?;
                        e.constant_fold();
                        if e.is_primitive() {
                            args.push(e);
                        } else {
                            let t = e.get_type()?;
                            new_instructions.push(Instr::PushExpr(e));
                            match t {
                                ExprType::Int => args.push(Expr::VarInt(stoff)),
                                ExprType::Float => args.push(Expr::VarFloat(stoff as f32)),
                                ExprType::String => {
                                    return Err(Error::Simple(
                                        "Can't push string onto the stack".to_owned(),
                                    ))
                                }
                            }
                            stoff -= 1;
                        }
                    }
                    let ins_found = crate::ecl_instructions::matching_ins_sep(name, &args)?;
                    let ins_opcode = match ins_found {
                        MatchInsResult::NoMatch(near_matches) => {
                            println!("No instruction matching {}", i.signature()?);
                            for nm in near_matches {
                                match nm.mt {
                                    MatchType::StringInVarargs => println!(
                                        "- Found instruction {} but string was used in vararg",
                                        nm.id.signature()
                                    ),
                                    MatchType::NameAndArgCountMatch => println!(
                                "- Found instruction {} with same name and number of arguments",
                                nm.id.signature()
                            ),
                                    MatchType::NameMatch => {
                                        println!(
                                            "- Found instruction {} with same name",
                                            nm.id.signature()
                                        )
                                    }
                                    _ => {}
                                }
                            }
                            return Err(Error::Simple(
                                "Couldn't resolve instruction call".to_owned(),
                            ));
                        }
                        MatchInsResult::MatchVA(oc, _va) => oc,
                        MatchInsResult::Match(oc) => oc,
                    };

                    let new_name = String::from(format!("ins_{ins_opcode}"));
                    // if vararg, insert type markers
                    new_instructions.push(Instr::Call(new_name, args));
                }
                _ => new_instructions.push(i.clone()),
            }
        }
        self.instructions = new_instructions;
        Ok(())
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

    pub fn process(&mut self) -> Result<(), Error> {
        let mut lbl_seed = 0usize;
        self.replace_vars()?;
        self.instructions = builtin_idents::replace(&self.instructions)?;
        self.instructions = if_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed)?;
        self.instructions = loop_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed);
        self.instructions =
            while_construct::desugar_bloc(&self, &self.instructions, &mut lbl_seed)?;
        // desugar other
        // maybe resolve variables before flattening anything.
        self.check_if_sub_returns();
        self.check_expressions()?;
        self.resolve_push_expr()?;
        self.resolve_labels();
        // optimize jump chain and remove dead code at some point
        // resolve other identifiers: vars, constants ... (right now there is none)
        Ok(())
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

fn resolve_param(typ: &Vec<String>, args: &Vec<AstNode>) -> Result<AstNode, Error> {
    if typ.len() != 1 {
        return Err(Error::Grammar(
            "Param command is composed of 1 sub command".to_owned(),
        ));
    }
    if args.len() != 1 {
        return Err(Error::Grammar("Sub command takes 1 parameter".to_owned()));
    }
    let typ = &typ[0];
    Ok(AstNode::Param(match &typ[..] {
        "Int" => Param::Int(args[0].clone().token().id()),
        "Float" => Param::Float(args[0].clone().token().id()),
        _ => {
            return Err(Error::Grammar(format!(
                "Unknown Param subcommand {}",
                &typ[..]
            )))
        }
    }))
}

fn resolve_sub(typ: &Vec<String>, args: &Vec<AstNode>) -> Result<AstNode, Error> {
    if !typ.is_empty() {
        return Err(Error::Grammar("Sub command has no subcommand".to_owned()));
    }
    if args.len() != 3 {
        return Err(Error::Grammar("Sub command takes 3 parameters".to_owned()));
    }
    let val = args[0].clone().token().id();
    let param_list = args[1].clone().list();
    let ins_list = args[2].clone().list();
    Ok(AstNode::Sub(Sub {
        name: val.clone(),
        params: param_list.into_iter().map(|n| n.param()).collect(),
        instructions: ins_list.into_iter().map(|n| n.instr()).collect(),
    }))
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Sub", resolve_sub);
    resolver.add_func("Param", resolve_param);
}
