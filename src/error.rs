use crate::lexer::SourceFile;
use crate::lexer::Token;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Simple(String),
    BackEnd(String),
    Grammar(String),
    ShouldNeverBeThere,
}

pub fn report_error(tok: &Token<&str>, cf: &SourceFile, text: &str) {
    let loc_str = format!("{}", tok.loc);
    let code = cf.get_line(tok.loc.line);
    println!("{}: {}", loc_str, code);
    // squiglies
    let mut squiglies = String::new();
    let col = tok.loc.span.start + loc_str.len() + 2;
    let len = tok.loc.span.end - tok.loc.span.start;
    squiglies.push_str(&" ".repeat(col));
    squiglies.push_str(&"_".repeat(len));
    println!("{}", squiglies);
    println!("{}", text);
}
