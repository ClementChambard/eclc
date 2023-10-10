#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Location<'t> {
    pub src_name: &'t str,
    pub line: usize,
    pub span: std::ops::Range<usize>,
}

impl<'t> std::fmt::Display for Location<'t> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.src_name, self.line, self.span.start)
    }
}

#[derive(Debug)]
pub struct SourceFile<'t> {
    pub filename: &'t str,
    pub content: String,
    line_sizes: Vec<usize>,
}

impl<'t> SourceFile<'t> {

    fn calculate_line_sizes(content: &str) -> Vec<usize> {
        content.split('\n').map(|s| s.len() + 1).collect()
    }

    pub fn open(filename: &'t str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(filename)?;
        Ok(Self {
            filename,
            line_sizes: Self::calculate_line_sizes(&content),
            content,
        })
    }

    pub fn remaining(&'t self, pos: usize) -> &'t str {
        &self.content[pos..]
    }

    pub fn span(&'t self, range: std::ops::Range<usize>) -> &'t str {
        &self.content[range]
    }

    pub fn len(&self) -> usize { self.content.len() }

    pub fn range_to_location(&self, range: std::ops::Range<usize>) -> Location<'t> {
        let len = range.end - range.start;
        let mut pos = range.start;
        let mut line = 0;
        for l in &self.line_sizes {
            if pos >= *l {
                line += 1;
                pos -= *l;
            } else {
                break;
            }
        }
        Location {
            src_name: self.filename,
            line,
            span: pos..pos + len
        }
    }

    pub fn get_line(&'t self, i: usize) -> &'t str {
        if i > self.line_sizes.len() {
            panic!("Out of range line {i}");
        }
        let start = self.line_sizes[..i].iter().sum();
        let len = self.line_sizes[i] - 1;
        &self.content[start..start + len]
    }
}

impl<'t> From<&'t str> for SourceFile<'t> {
    fn from(s: &str) -> SourceFile<'t> {
        SourceFile {
            filename: "dummy",
            content: s.to_owned(),
            line_sizes: Self::calculate_line_sizes(s),
        }
    }
}
