use super::*;

fn resolve_list(typ: &Vec<String>, args: &Vec<AstNode>) -> AstNode {
    assert!(typ.len() == 1);
    let typ = &typ[0];
    match &typ[..] {
        "empty" => AstNode::List(vec![]),
        "prepend" => {
            assert!(args.len() == 2);
            let mut list = args[0].clone().list();
            let val = args[1].clone();
            if !val.is_none() {
                list.insert(0, val);
            }
            AstNode::List(list)
        }
        f => {
            panic!("Unknown list function {f}");
        }
    }
}

pub fn fill_executor(resolver: &mut AstResolver<AstNode>) {
    resolver.add_func("List", resolve_list);
}
