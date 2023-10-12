pub fn string(s: &str) -> String {
    // remove "" and do escape proceed to escape.
    let mut s = s
        .strip_prefix('"')
        .unwrap()
        .strip_suffix('"')
        .unwrap()
        .to_string();
    s = s.replace("\\n", "\n");
    s = s.replace("\\t", "\t");
    s = s.replace("\\\"", "\"");
    s = s.replace("\\'", "'");
    // TODO: all escape sequences
    s.to_string()
}

pub fn int(s: &str) -> i32 {
    if s == "true" {
        return 1;
    } else if s == "false" {
        return 0;
    }
    // TODO: check all cases
    s.parse().unwrap()
}

pub fn float(s: &str) -> f32 {
    let mut s = s;
    if s.ends_with('f') {
        s = s.strip_suffix('f').unwrap();
    }
    s.parse().unwrap()
}
