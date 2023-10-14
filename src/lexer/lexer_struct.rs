use super::LexerBuilder;
use super::SourceFile;
use super::Tokens;
use regex::{Regex, RegexSet};

#[derive(Debug)]
pub struct Lexer<K> {
    kinds: Vec<Option<K>>,
    regexes: Vec<Regex>,
    regex_set: RegexSet,
    eof: Option<K>,
    error: K,
}

impl<K: Copy> Lexer<K> {
    pub fn new(
        kinds: Vec<Option<K>>,
        regexes: Vec<Regex>,
        regex_set: RegexSet,
        eof: Option<K>,
        error: K,
    ) -> Self {
        Self {
            kinds,
            regexes,
            regex_set,
            eof,
            error,
        }
    }

    pub fn eof_token(&self) -> Option<K> {
        self.eof
    }
    pub fn error(&self) -> K {
        self.error.clone()
    }
    pub fn get_regexes(&self) -> &Vec<Regex> {
        &self.regexes
    }
    pub fn get_regex_set(&self) -> &RegexSet {
        &self.regex_set
    }
    pub fn kind(&self, i: usize) -> Option<K> {
        self.kinds[i]
    }

    pub fn builder<'r>() -> LexerBuilder<'r, K> {
        LexerBuilder::new()
    }

    pub fn tokens<'l>(&'l self, source: &SourceFile) -> Tokens<'l, K> {
        Tokens::new(self, source)
    }
}
