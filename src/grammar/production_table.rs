use super::Symbol;

pub struct ProductionTableEntry {
    nt: String,
    tok: String,
    pub produces: Vec<Symbol>,
}

use std::io::Write;

impl ProductionTableEntry {
    pub fn new(nt: &str, tok: &str, produces: Vec<Symbol>) -> Self {
        Self {
            nt: nt.to_string(),
            tok: tok.to_string(),
            produces,
        }
    }

    fn _gen_rust<W: Write>(&self, buf: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        writeln!(buf, "    table.push(ProductionTableEntry::new(")?;
        writeln!(buf, "        \"{}\",", self.nt)?;
        writeln!(buf, "        \"{}\",", self.tok)?;
        write!(buf, "        vec![")?;
        for (i, s) in self.produces.iter().enumerate() {
            s._gen_rust(buf)?;
            if i < self.produces.len() - 1 {
                write!(buf, ", ")?;
            }
        }
        writeln!(buf, "]")?;
        writeln!(buf, "    ));")?;
        Ok(())
    }
}

pub type ProductionTable = Vec<ProductionTableEntry>;

pub fn get_production_table_entry(
    table: &ProductionTable,
    nt: &str,
    tok: &str,
) -> Option<Vec<Symbol>> {
    Some(
        table
            .iter()
            .find(|entry| entry.nt == nt && entry.tok == tok)?
            .produces
            .clone(),
    )
}

use std::io::BufWriter;
pub fn _gen_rust_for_production_table<W: Write>(
    buf: &mut BufWriter<W>,
    table: &ProductionTable,
) -> Result<(), std::io::Error> {
    writeln!(buf, "pub fn gen_table() -> ProductionTable {{")?;
    writeln!(buf, "    let mut table = vec![];")?;
    for e in table {
        e._gen_rust(buf)?;
    }
    writeln!(buf, "    table")?;
    writeln!(buf, "}}")?;
    Ok(())
}

impl std::fmt::Display for ProductionTableEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{} -> ", self.nt, self.tok)?;
        for s in &self.produces {
            write!(f, "{} ", s)?;
        }
        Ok(())
    }
}
