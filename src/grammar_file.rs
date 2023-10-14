mod parse_rules;

pub use parse_rules::parse_rules;

use std::collections::HashMap;

pub type RulePriorities = HashMap<String, i32>;

#[derive(Debug, Clone)]
struct LineData {
    content: String,
    _line_number: usize,
    priorities: RulePriorities,
}

#[derive(Debug)]
pub struct TokenDecl {
    in_file: String,
    regex: Option<String>,
    is_eof: bool,
    escape: bool,
}

// todo: macros

impl TokenDecl {
    fn from_directive(directive: &str) -> Result<Self, &str> {
        // todo: parse correctly
        let mut splits = directive.split(' ');
        if let Some(s) = splits.next() {
            if s != "!token" {
                return Err("token directive should start with \"!token \"");
            }
        } else {
            return Err("token directive should start with \"!token \"");
        }
        let Some(in_file) = splits.next() else {
            return Err("token directive needs at least an in-file version");
        };
        let mut regex = None;
        let mut is_eof = false;
        if let Some(second) = splits.next() {
            if second == "$" {
                is_eof = true;
            } else {
                if second != "=>" {
                    return Err("token directive error");
                }
                regex = Some(splits.collect());
            }
        }
        Ok(TokenDecl {
            in_file: in_file.to_string(),
            escape: regex.is_none(),
            regex,
            is_eof,
        })
    }
}

impl LineData {
    fn vec_from_str(s: &str) -> Vec<Self> {
        s.split('\n')
            .enumerate()
            .map(|(i, l)| Self {
                content: l.trim().to_owned(),
                _line_number: i,
                priorities: RulePriorities::new(),
            })
            .filter(|l| !l.content.is_empty())
            .collect()
    }
}

#[derive(Debug, Default)]
pub struct GrammarFile {
    lines: Vec<LineData>,
    tokens: Vec<TokenDecl>,
    ignore_decls: Vec<String>,
    next_line_prio: RulePriorities,
    line: usize,
    pos_in_line: usize,
}

impl GrammarFile {
    fn token(&mut self, tok_directive: &str) {
        let tok = TokenDecl::from_directive(tok_directive).unwrap();
        self.tokens.push(tok);
    }

    fn ignore(&mut self, ignore_directive: &str) {
        let s = ignore_directive.strip_prefix("!ignore ").unwrap();
        self.ignore_decls.push(s.to_owned());
    }

    fn prio(&mut self, prio_directive: &str) {
        let mut splits = prio_directive.split(' ');
        assert_eq!(splits.next().unwrap(), "!prio");
        let tok = splits.next().unwrap();
        let prio: i32 = splits.next().unwrap().parse().unwrap();
        self.next_line_prio.insert(tok.to_owned(), prio);
    }

    pub fn _gen_rust(&self) {
        println!("pub fn init_lexer<'a>() -> Lexer<&'a str> {{");
        println!("    Lexer::builder()");
        for i in &self.ignore_decls {
            println!(
                "        .ignore(\"{}\")",
                &i[..].replace('\\', "\\\\").replace('"', "\\\"")
            );
        }
        for t in &self.tokens {
            if t.is_eof {
                println!("        .eof(\"{}\")", &t.in_file[..]);
                continue;
            }
            let re = match &t.regex {
                Some(re) => re,
                None => &t.in_file,
            };
            println!(
                "        .token(\"{}\", \"{}\", {})",
                &re.replace('\\', "\\\\").replace('"', "\\\""),
                &t.in_file[..].replace('\\', "\\\\").replace('"', "\\\""),
                t.escape
            );
        }
        println!("        .build().unwrap()");
        println!("}}");
    }

    pub fn from_file(filename: &str) -> Result<Self, std::io::Error> {
        Ok(Self::preprocess(&std::fs::read_to_string(filename)?))
    }

    pub fn preprocess(content: &str) -> Self {
        let mut f = Self::default();
        let mut new_lines = vec![];

        let lines = LineData::vec_from_str(content);

        for l in lines {
            let content = &l.content;
            if content.starts_with('!') {
                match content.split(' ').next().unwrap() {
                    "!token" => f.token(content),
                    "!ignore" => f.ignore(content),
                    "!prio" => f.prio(content),
                    _ => {} // treated as a comment
                }
            } else {
                let mut new_l: LineData = l.clone();
                new_l.priorities = f.next_line_prio;
                f.next_line_prio = RulePriorities::new();
                new_lines.push(new_l);
            }
        }
        f.lines = new_lines;
        f
    }

    pub fn consume(&mut self) {
        if self.line >= self.lines.len() || self.pos_in_line > self.lines[self.line].content.len() {
            return;
        }
        self.pos_in_line += 1;
        if self.pos_in_line >= self.lines[self.line].content.len() {
            self.line += 1;
            self.pos_in_line = 0;
        }
    }

    pub fn eof(&self) -> bool {
        self.line >= self.lines.len() || self.pos_in_line > self.lines[self.line].content.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.line >= self.lines.len() || self.pos_in_line > self.lines[self.line].content.len() {
            return None;
        }
        if self.pos_in_line == self.lines[self.line].content.len() {
            return Some('\n');
        }
        self.lines[self.line].content.chars().nth(self.pos_in_line)
    }

    pub fn next(&mut self) -> Option<char> {
        let n = self.peek();
        self.consume();
        n
    }

    pub fn prio_here(&self) -> Option<RulePriorities> {
        if self.line >= self.lines.len() || self.pos_in_line > self.lines[self.line].content.len() {
            return None;
        }
        Some(self.lines[self.line].priorities.clone())
    }

    pub fn read_while<T: Fn(char) -> bool>(&mut self, f: T) -> String {
        let mut s = String::new();
        let mut c = match self.peek() {
            Some(c) => c,
            None => return s,
        };
        while f(c) {
            s.push(c);
            self.consume();
            c = match self.peek() {
                Some(c) => c,
                None => break,
            };
        }
        s
    }

    pub fn read_while_between(&mut self, open: char, close: char) -> String {
        let mut s = String::new();
        let mut level = 0;
        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return s,
            };
            if c == open {
                level += 1;
            } else if c == close {
                level -= 1;
            }
            s.push(c);
            self.consume();
            if level <= 0 {
                break;
            }
        }
        s
    }

    pub fn consume_while<T: Fn(char) -> bool>(&mut self, f: T) {
        let mut c = match self.peek() {
            Some(c) => c,
            None => return,
        };
        while f(c) {
            self.consume();
            c = match self.peek() {
                Some(c) => c,
                None => break,
            };
        }
    }

    pub fn lexer(&self) -> crate::lexer::Lexer<&str> {
        let mut lb = crate::lexer::Lexer::builder();
        for t in &self.tokens {
            if t.is_eof {
                lb = lb.eof(&t.in_file[..]);
                continue;
            }
            let re = match &t.regex {
                Some(re) => re,
                None => &t.in_file,
            };
            lb = lb.token(re, &t.in_file[..], t.escape);
        }
        lb = lb.error("ERROR");
        for i in &self.ignore_decls {
            lb = lb.ignore(&i[..]);
        }
        lb.build().expect("Incorrect lexer definition")
    }
}
