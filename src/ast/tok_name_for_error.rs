pub fn tok_name_for_error(tok: &str) -> String {
    match tok {
        "rb" => "`}`".to_owned(),
        "lb" => "`{`".to_owned(),
        "id" => "identifier".to_owned(),
        "str" => "string litteral".to_owned(),
        "int" => "integer litteral".to_owned(),
        "float" => "float litteral".to_owned(),
        "EOF" => "`<eof>`".to_owned(),
        a if a.starts_with("kw_") => {
            format!("keyword `{}`", a.strip_prefix("kw_").unwrap())
        }
        _ => format!("`{}`", tok),
    }
}
