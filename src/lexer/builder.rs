use super::Lexer;

pub use regex::Error;
use regex::{Regex, RegexSet};

pub struct LexerBuilder<'r, K> {
    regexes: Vec<&'r str>,
    kinds: Vec<Option<K>>,
    escape: Vec<bool>,
    eof: Option<K>,
    error: Option<K>,
}

impl<'r, K: Copy> Default for LexerBuilder<'r, K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'r, K: Copy> LexerBuilder<'r, K> {
    pub fn new() -> Self {
        LexerBuilder {
            regexes: Vec::new(),
            kinds: Vec::new(),
            escape: Vec::new(),
            eof: None,
            error: None,
        }
    }

    pub fn token(mut self, re: &'r str, kind: K, escape: bool) -> Self {
        self.regexes.push(re);
        self.kinds.push(Some(kind));
        self.escape.push(escape);
        self
    }

    pub fn ignore(mut self, re: &'r str) -> Self {
        self.regexes.push(re);
        self.kinds.push(None);
        self.escape.push(false);
        self
    }

    pub fn eof(mut self, kind: K) -> Self {
        self.eof = Some(kind);
        self
    }

    pub fn error(mut self, kind: K) -> Self {
        self.error = Some(kind);
        self
    }

    pub fn build(self) -> Result<Lexer<K>, Error> {
        let regexes = self.regexes.into_iter().zip(self.escape).map(|(r, e)| {
            if e {
                format!("^{}", &regex::escape(r))
            } else {
                format!("^{}", r)
            }
        });

        let regex_set = RegexSet::new(regexes)?;
        let mut regexes = Vec::new();
        for pattern in regex_set.patterns() {
            regexes.push(Regex::new(pattern)?);
        }

        Ok(Lexer::new(
            self.kinds,
            regexes,
            regex_set,
            self.eof,
            self.error.unwrap(),
        ))
    }
}
