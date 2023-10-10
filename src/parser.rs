use crate::error::report_error;
use crate::grammar::Grammar;
use crate::grammar::get_production_table_entry;
use crate::grammar::Symbol;
use crate::lexer::Tokens;

pub mod ast;
use ast::*;

struct NodeAndTimes<'a, 'b> {
    pub node: Node<'a, 'b>,
    pub times: usize,
}

pub fn parse<'a, 'b>(grammar: &Grammar, tokens: Tokens<'a, 'b, &'b str>, first_nt: &str) -> Option<Node<'a, 'b>> {
    let mut tokens = tokens;
    let parsing_table = grammar.fill_ll1_production_table();
    let mut symbols_to_derive = vec![Symbol::NT(first_nt.to_string())];
    let mut cur_token_opt = tokens.next();

    let mut node_stack: Vec<NodeAndTimes> = vec![];
    let mut ret = None;

    while !symbols_to_derive.is_empty() {
        let Some(ref cur_token) = cur_token_opt else { break; };
        let s = &symbols_to_derive[0];
        match s {
            Symbol::NT(nt) => {
                if let Some(parsing_table_entry) = get_production_table_entry(&parsing_table, nt, cur_token.kind) {
                    let mut rule_symbols = parsing_table_entry;
                    node_stack.push(NodeAndTimes {
                        node: Node::NT(nt.clone(), vec![]),
                        times: rule_symbols.len(),
                    });
                    rule_symbols.extend(symbols_to_derive.into_iter().skip(1));
                    symbols_to_derive = rule_symbols;
                } else {
                    report_error(cur_token, tokens.get_sourcefile(),
                        &format!("Production table entry not found for token \"{}\" at non terminal \"{}\"",
                            cur_token.kind, nt));
                    return None;
                }
            },
            Symbol::T(t) => {
                if t == cur_token.kind {
                    node_stack.push(NodeAndTimes {
                        node: Node::T(cur_token.clone()),
                        times: 0,
                    });
                    cur_token_opt = tokens.next();
                    symbols_to_derive = symbols_to_derive[1..].to_vec();
                } else {
                    report_error(cur_token, tokens.get_sourcefile(),
                        &format!("Expected token \"{}\", got \"{}\"", t, cur_token.kind));
                    return None;
                }
            }
        }
        loop {
            if let Some(n) = node_stack.iter().last() {
                if n.times == 0 {
                    let n = node_stack.pop().unwrap();
                    if let Some(ref mut n2) = node_stack.iter_mut().last() {
                        let Node::NT(_, ref mut v) = n2.node else { panic!() };
                        n2.times -= 1;
                        v.push(n.node);
                    } else {
                        ret = Some(n.node);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    if symbols_to_derive.is_empty() {
        println!("Parsed successfully");
        ret
    } else {
        println!("Parsing early exit");
        ret
    }
}
