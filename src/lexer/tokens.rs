use crate::error::report_error;

use super::Lexer;
use super::SourceFile;
use super::Token;

#[derive(Debug)]
pub struct Tokens<'l, K> {
    lexer: &'l Lexer<K>,
    source: SourceFile,
    position: usize,
    eof: bool,
}

impl<'l, K> Tokens<'l, K> {
    pub fn new(lexer: &'l Lexer<K>, source: &SourceFile) -> Self {
        Self {
            lexer,
            source: source.clone(),
            position: 0,
            eof: false,
        }
    }
}

impl<'l, K: Copy> Iterator for Tokens<'l, K> {
    type Item = Token<K>;

    fn next(&mut self) -> Option<Token<K>> {
        loop {
            if self.eof {
                return None;
            }
            if self.position == self.source.len() {
                self.eof = true;
                return Some(Token {
                    kind: self.lexer.eof_token()?,
                    loc: self.source.range_to_location(self.position..self.position),
                    text: "".to_owned(),
                });
            }

            let string = self.source.remaining(self.position);
            let match_set = self.lexer.get_regex_set().matches(string);
            let result = match_set
                .into_iter()
                .map(|i: usize| {
                    let m = self.lexer.get_regexes()[i].find(string).unwrap();
                    assert!(m.start() == 0);
                    (m.end(), i)
                })
                .next_back();
            let (len, i) = if let Some((a, b)) = result {
                (a, b)
            } else {
                report_error(
                    &self
                        .source
                        .range_to_location(self.position..self.position + 1),
                    &format!("unknown symbol `{}`: ignoring character.", &string[0..1]),
                );
                self.position += 1;
                return self.next();
            };

            let loc = self
                .source
                .range_to_location(self.position..self.position + len);
            let text = self.source.span(self.position..self.position + len);
            self.position += len;
            if let Some(kind) = self.lexer.kind(i) {
                return Some(Token {
                    kind,
                    loc,
                    text: text.to_string(),
                });
            }
        }
    }
}
