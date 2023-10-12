use super::*;

fn resolve_list(typ: &[String], args: &[AstNode]) -> Result<AstNode, Error> {
    if typ.len() != 1 {
        return Err(Error::Grammar(
            "List command is composed of 1 subcommand".to_owned(),
        ));
    }
    let typ = &typ[0];
    match &typ[..] {
        "empty" => Ok(AstNode::List(vec![])),
        "prepend" => {
            if args.len() != 2 {
                return Err(Error::Grammar(
                    "List::prepend sub command takes 2 params".to_owned(),
                ));
            }

            let mut list = args[0].clone().list_or(Error::Grammar(
                "Param0 of List::prepend should be a list".to_owned(),
            ))?;
            let val = args[1].clone();
            if !val.is_none() {
                list.insert(0, val);
            }
            Ok(AstNode::List(list))
        }
        f => Err(Error::Grammar(format!("Unknown List subcommand {f}"))),
    }
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("List", resolve_list);
}
