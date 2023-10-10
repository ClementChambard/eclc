use super::source_file::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'t, K> {
    pub kind: K,
    pub loc: Location<'t>,
    pub text: &'t str,
}

impl<'t, K: std::fmt::Display> std::fmt::Display for Token<'t, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} {}", self.loc, self.kind, self.text)
    }
}
