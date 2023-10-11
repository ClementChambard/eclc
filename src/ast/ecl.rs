use super::*;

#[derive(Debug, Clone)]
pub struct Ecl {
    pub ecli: Vec<String>,
    pub anmi: Vec<String>,
    pub subs: Vec<Sub>,
}

impl Ecl {
    pub fn process(&mut self) {
        for s in &mut self.subs {
            s.process();
        }
    }
}

fn resolve_ecl(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.is_empty());
    assert!(args.len() == 3);
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
    AstNode::Ecl(Ecl { ecli, anmi, subs })
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("Ecl", resolve_ecl);
}
