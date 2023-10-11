use crate::error::Error;

use super::*;

#[derive(Debug, Clone)]
pub struct Ecl {
    pub ecli: Vec<String>,
    pub anmi: Vec<String>,
    pub subs: Vec<Sub>,
}

impl Ecl {
    pub fn process(&mut self) -> Result<(), Error> {
        for s in &mut self.subs {
            s.process()?;
        }
        Ok(())
    }
}

fn resolve_ecl(typ: &Vec<String>, args: &Vec<AstNode>) -> Result<AstNode, Error> {
    if !typ.is_empty() {
        return Err(Error::Grammar("Ecl command has no subcommands".to_owned()));
    }
    if args.len() != 3 {
        return Err(Error::Grammar("Ecl commands takes 3 params".to_owned()));
    }
    let ecli = args[0]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.token().strn())
        .collect();
    let anmi = args[1]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.token().strn())
        .collect();
    let subs = args[2]
        .clone()
        .list()
        .into_iter()
        .map(|n| n.sub())
        .collect();
    Ok(AstNode::Ecl(Ecl { ecli, anmi, subs }))
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Ecl", resolve_ecl);
}
