#[macro_use]
extern crate lazy_static;

use crossterm::style::Stylize;
use std::{io::Write, sync::Mutex};

mod ast;
mod code_gen;
mod ecl_instructions;
mod error;
mod grammar;
mod grammar_file;
mod lexer;
mod parser;
mod print_bytes;

use ast::AstNode;
use error::{report_error_simple, Error};
use grammar::Grammar;
use lexer::{Lexer, SourceFile};
use parser::ast::AstResolver;

// Define a sample struct
#[derive(Default)]
struct Globals {
    code_file: Option<SourceFile>,
}

// Create a global mutable variable using lazy_static and Mutex
lazy_static! {
    static ref GLOBAL: Mutex<Globals> = Mutex::new(Globals::default());
}

fn gen_file(
    fname: &str,
    lexer: &Lexer<&str>,
    grammar: &Grammar,
    ast_resolver: &AstResolver<AstNode>,
) -> Result<(), Error> {
    // Open code file
    GLOBAL.lock().unwrap().code_file = Some(lexer::SourceFile::open(fname).map_err(Error::IO)?);

    let tokens = {
        let lock = GLOBAL.lock().unwrap();
        lexer.tokens(lock.code_file.as_ref().unwrap())
    };

    // Parse code
    let mut node = ast_resolver
        .resolve(
            &parser::parse(grammar, tokens, "Ecl")
                .ok_or(Error::Simple("Could not parse node: Aborting".to_owned()))?,
            &[],
        )?
        .ecl();
    // println!("{:#?}", node);

    // Process code for binary generation
    node.process()?;
    // println!("{:#?}", node);

    // generate binary
    let bytes = code_gen::generate(&node);
    // print_bytes::pr(&bytes);
    std::fs::File::create("out.ecl")
        .map_err(Error::IO)?
        .write_all(&bytes)
        .map_err(Error::IO)?;

    Ok(())
}

fn main_sub() -> Result<(), Error> {
    // Read grammar
    let mut gf = grammar_file::GrammarFile::from_file("test.grammar").map_err(Error::IO)?;
    let rulestrings = grammar_file::parse_rules(&mut gf);
    let mut grammar = grammar::Grammar::from_rule_string(rulestrings);
    grammar.calculate_first_sets();
    grammar.calculate_follow_sets();

    if !crate::grammar::is_ll1_grammar(&grammar) {
        return Err(Error::Grammar("The grammar is not LL1".to_owned()));
    }

    let lexer = gf.lexer();
    let mut ast_resolver = parser::ast::AstResolver::default();
    ast_resolver.set_ast_prod(grammar.get_ast_prod());
    ast::fill_executor(&mut ast_resolver);

    let src_name = "test.code";
    let bin_name = "out.ecl";

    println!(
        "   {} `{}` from source `{}`",
        "Compiling".bold().with(crossterm::style::Color::Green),
        bin_name,
        src_name
    );
    if let Err(_) = gen_file(src_name, &lexer, &grammar, &ast_resolver) {
        report_error_simple(&format!(
            "could not compile `{}` (bin \"{}\") due to previous error",
            src_name, bin_name
        ));
    } else {
        println!(
            "    {} building `{}`",
            "Finished".bold().with(crossterm::style::Color::Green),
            bin_name
        );
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_sub() {
        match e {
            Error::Simple(s) => {
                println!("There was an error and the compilation couldn't finish:\n{s}");
            }
            Error::Grammar(s) => {
                println!("Grammar error: {s}");
            }
            Error::BackEnd(s) => {
                println!("BackEnd error: {s}");
            }
            Error::ShouldNeverBeThere => {
                println!("An error occured that should have never occured");
            }
            Error::IO(e) => {
                println!("IO error: {}", e);
            }
        }
    }
}
