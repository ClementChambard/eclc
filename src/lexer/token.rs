use super::source_file::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<K> {
    pub kind: K,
    pub loc: Location,
    pub text: String,
}

impl<K: std::fmt::Display> std::fmt::Display for Token<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} {}", self.loc, self.kind, self.text)
    }
}
