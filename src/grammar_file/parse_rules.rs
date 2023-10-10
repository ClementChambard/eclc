use super::{GrammarFile, RulePriorities};
use crate::grammar::rules::RuleStrings;

fn parse_rule(body: &str, nt: &str, prio: RulePriorities, ast: &str) -> RuleStrings {
    let right = body
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    RuleStrings {
        left: nt.to_owned(),
        right,
        prio,
        ast: crate::parser::ast::parse_ast_def(ast),
    }
}

pub fn parse_rules(f: &mut GrammarFile) -> Vec<RuleStrings> {
    let mut rules = vec![];

    while !f.eof() {
        f.consume_while(char::is_whitespace);

        // non terminal for the rule:
        let nt = f.read_while(|c| !c.is_whitespace());
        f.consume_while(char::is_whitespace);

        // expect "::="
        let sym = vec![f.next().unwrap(), f.next().unwrap(), f.next().unwrap()];
        assert_eq!(sym, vec![':', ':', '=']);

        // can have multiple rule body
        loop {
            f.consume_while(char::is_whitespace);
            // rule has the priority of the first line it spans:
            let prio = f.prio_here().unwrap();
            // rule body
            let rule_body = f.read_while(|c| c != '{');
            let rule_ast_construct = f.read_while_between('{', '}');

            rules.push(parse_rule(&rule_body, &nt, prio, &rule_ast_construct));

            f.consume_while(char::is_whitespace);
            if let Some(c) = f.peek() {
                if c != '|' {
                    break;
                }
                f.consume();
            } else {
                break;
            }
        }
    }

    rules
}
