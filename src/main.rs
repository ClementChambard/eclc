use std::io::Write;

mod ast;
mod code_gen;
mod error;
mod grammar;
mod grammar_file;
mod lexer;
mod parser;
mod print_bytes;

use error::Error;

fn main() -> Result<(), Error> {
    // Read grammar
    let mut gf = grammar_file::GrammarFile::from_file("test.grammar").map_err(|e| Error::IO(e))?;
    let rulestrings = grammar_file::parse_rules(&mut gf);
    let mut grammar = grammar::Grammar::from_rule_string(rulestrings);
    grammar.calculate_first_sets();
    grammar.calculate_follow_sets();
    crate::grammar::is_ll1_grammar(&grammar);
    let lexer = gf.lexer();
    let mut ast_resolver = parser::ast::AstResolver::default();
    ast_resolver.set_ast_prod(grammar.get_ast_prod());
    ast::fill_executor(&mut ast_resolver);

    // Open code file
    let src = lexer::SourceFile::open("test.code").map_err(|e| Error::IO(e))?;

    // Parse code
    let mut node = ast_resolver
        .resolve(
            &parser::parse(&grammar, lexer.tokens(&src), "Ecl")
                .ok_or(Error::Simple("Could not parse node: Aborting".to_owned()))?,
            &vec![],
        )
        .ecl();
    println!("{:#?}", node);

    // Process code for binary generation
    node.process();
    println!("{:#?}", node);

    // generate binary
    let bytes = code_gen::generate(&node);
    print_bytes::pr(&bytes);
    std::fs::File::create("out.ecl")
        .map_err(|e| Error::IO(e))?
        .write_all(&bytes)
        .map_err(|e| Error::IO(e))?;

    Ok(())
}
