use super::{Node, AstResolver, NeededForAstNode};

#[derive(Debug, Clone)]
pub enum AstDef {
    Der(Der),
    Fun(Vec<String>, Vec<Der>),
}

#[derive(Debug, Clone)]
pub enum Der {
    Child(usize),
    ChildDer(usize, Vec<AstDef>),
    Param(usize),
}

impl Der {
    pub fn execute<N: NeededForAstNode>(&self, resolver: &AstResolver<N>, children: &Vec<Node>, params: &Vec<N>) -> N {
        match self {
            Self::Child(n) => {
                resolver.resolve(&children[*n], &vec![])
            },
            Self::ChildDer(n, par) => {
                resolver.resolve(&children[*n], &par.iter().map(|p| p.execute(resolver, children, params)).collect())
            },
            Self::Param(n) => {
                params[*n].clone()
            },
        }
    }
}

impl AstDef {
    pub fn execute<N: NeededForAstNode>(&self, resolver: &AstResolver<N>, children: &Vec<Node>, params: &Vec<N>) -> N {
        match self {
            Self::Der(d) => d.execute(resolver, children, params),
            Self::Fun(fun_name, par) => {
                let par = par.iter().map(|p|  p.execute(resolver, children, params)).collect();
                resolver.executor.exec(fun_name, &par)
            },
        }
    }
}

trait EatSpace {
    fn eat_spaces(&self) -> &Self;
}

impl EatSpace for str {
    fn eat_spaces(&self) -> &str {
        let mut skip = 0;
        for c in self.chars() {
            if !c.is_whitespace() {
                break;
            }
            skip += 1;
        }
        &self[skip..]
    }
}

pub fn parse_ast_def(s: &str) -> AstDef {
    let mut it = s.chars();
    assert!(it.next() == Some('{'));
    let s = &s[1..].eat_spaces();
    let AstDefResult{res, next_str: s} = parse_ast_def_in(s);
    let s = s.eat_spaces();
    assert!(s.len() == 1 && s.chars().next() == Some('}'));
    res
}

struct IdentResult<'a> {
    res: String,
    next_str: &'a str,
}

fn read_ident<'a>(s: &'a str) -> IdentResult<'a> {
    let mut it = s.chars();
    let mut out = String::new();
    let c = it.next().expect("expected first char when reading ident");
    if !c.is_alphabetic() && c != '_' {
        panic!("expected letter or '_' when reading ident (got {c})");
    }
    out.push(c);
    for c in it {
        if !c.is_alphanumeric() && c != '_' {
            break;
        }
        out.push(c);
    }
    IdentResult {
        next_str: &s[out.len()..],
        res: out,
    }
}

struct AstFunResult<'a> {
    res: Vec<String>,
    next_str: &'a str,
}

fn parse_ast_fun<'a>(s: &'a str) -> AstFunResult<'a> {
    let mut out: Vec<String> = Vec::new();
    let mut st = s;
    loop {
        let IdentResult{res, next_str} = read_ident(st);
        out.push(res);
        st = next_str.eat_spaces();
        if st.starts_with("::") {
            st = st.strip_prefix("::").unwrap();
        } else {
            break;
        }
    }
    AstFunResult{ res: out, next_str: st }
}

fn get_num(s: &str) -> (usize, usize) {
    let mut out = String::new();
    for c in s.chars() {
        if !c.is_numeric() {
            break;
        }
        out.push(c);
    }
    (out.parse().unwrap(), out.len())
}

struct AstDefResult<'a> {
    res: AstDef,
    next_str: &'a str,
}

fn parse_ast_def_in<'a>(s: &'a str) -> AstDefResult<'a> {
    // either Der or Fn
    let s = s.eat_spaces();
    if s.starts_with('$') {
        let s = &s[1..];
        if s.starts_with("param") {
            let s = s.strip_prefix("param").unwrap();
            let (val, len) = get_num(s);
            let s = &s[len..].eat_spaces();
            AstDefResult{res: AstDef::Der(Der::Param(val)), next_str: s}
        } else {
            let (val, len) = get_num(s);
            let s = &s[len..].eat_spaces();
            if s.chars().next() == Some('.') {
                let s = &s[1..].eat_spaces();
                if !s.starts_with("derive") { panic!("should derive after '.'"); }
                let s = s.strip_prefix("derive").unwrap().eat_spaces();
                assert!(s.chars().next() == Some('('));
                let s = &s[1..].eat_spaces();
                let AstParamsResult{res: params, next_str: s} = parse_ast_params(s);
                assert!(s.chars().next() == Some(')'));
                AstDefResult{res: AstDef::Der(Der::ChildDer(val, params)), next_str: &s[1..].eat_spaces()}
            } else {
                AstDefResult{res: AstDef::Der(Der::Child(val)), next_str: s}
            }
        }
    } else {
        let AstFunResult{res: fun, next_str: s} = parse_ast_fun(s);
        assert!(s.chars().next() == Some('('));
        let s = &s[1..].eat_spaces();
        let AstParamsResult{res: params, next_str: s} = parse_ast_params(s);
        assert!(s.chars().next() == Some(')'));
        AstDefResult{res: AstDef::Fun(fun, params.into_iter().map(|p| match p {
            AstDef::Der(d) => d,
            _ => panic!("Should be der"),
        }).collect()), next_str: &s[1..].eat_spaces()}
    }
}

struct AstParamsResult<'a> {
    res: Vec<AstDef>,
    next_str: &'a str,
}

fn parse_ast_params<'a>(s: &'a str) -> AstParamsResult<'a> {
    let mut out: Vec<AstDef> = Vec::new();
    let mut st = s;
    if st.starts_with(')') {
        return AstParamsResult{ res: out, next_str: st };
    }
    loop {
        let AstDefResult{res, next_str} = parse_ast_def_in(st);
        out.push(res);
        st = next_str.eat_spaces();
        if st.starts_with(",") {
            st = st.strip_prefix(",").unwrap();
        } else {
            break;
        }
    }
    AstParamsResult{ res: out, next_str: st }
}
