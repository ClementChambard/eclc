use crate::ast::tok_name_for_error;
use crate::error::report_error;
use crate::error::report_error_ext;
use crate::grammar::get_production_table_entry;
use crate::grammar::get_production_table_tokens_for_nt;
use crate::grammar::Grammar;
use crate::grammar::Symbol;
use crate::lexer::Tokens;

pub mod ast;
use ast::*;

struct NodeAndTimes<'a> {
    pub node: Node<'a>,
    pub times: usize,
}

pub fn parse<'a>(grammar: &Grammar, tokens: Tokens<'a, &str>, first_nt: &str) -> Option<Node<'a>> {
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
                if let Some(parsing_table_entry) =
                    get_production_table_entry(&parsing_table, nt, cur_token.kind)
                {
                    let mut rule_symbols = parsing_table_entry;
                    node_stack.push(NodeAndTimes {
                        node: Node::NT(nt.clone(), vec![]),
                        times: rule_symbols.len(),
                    });
                    rule_symbols.extend(symbols_to_derive.into_iter().skip(1));
                    symbols_to_derive = rule_symbols;
                } else {
                    let expected_tokens = get_production_table_tokens_for_nt(&parsing_table, nt);
                    if expected_tokens.is_empty() {
                        report_error(
                            &cur_token.loc,
                            &format!("No production table entry for non terminal \"{}\"", nt),
                        );
                        return None;
                    }
                    let mut error_message = String::from("Expected one of : ");
                    let etl = expected_tokens.len();
                    for (i, t) in expected_tokens.into_iter().enumerate() {
                        if i == 0 {
                        } else if i == etl - 1 {
                            error_message.push_str(" or ");
                        } else {
                            error_message.push_str(", ");
                        }
                        error_message.push_str(&tok_name_for_error(t));
                    }
                    let under_error = error_message.clone();
                    error_message
                        .push_str(&format!(", found {}", tok_name_for_error(cur_token.kind)));
                    report_error_ext(&cur_token.loc, &error_message, &under_error);
                    return None;
                }
            }
            Symbol::T(t) => {
                if t == cur_token.kind {
                    node_stack.push(NodeAndTimes {
                        node: Node::T(cur_token.clone()),
                        times: 0,
                    });
                    cur_token_opt = tokens.next();
                    symbols_to_derive = symbols_to_derive[1..].to_vec();
                } else {
                    report_error_ext(
                        &cur_token.loc,
                        &format!(
                            "Expected {}, found {}",
                            tok_name_for_error(t),
                            tok_name_for_error(cur_token.kind)
                        ),
                        &format!("Expected {}", tok_name_for_error(t)),
                    );
                    return None;
                }
            }
        }
        while let Some(n) = node_stack.iter().last() {
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
