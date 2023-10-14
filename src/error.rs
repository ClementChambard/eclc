use std::ops::Range;

use crate::lexer::Location;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Simple(String),
    BackEnd(String),
    Grammar(String),
    ShouldNeverBeThere,
}

// error: expected item, found `:`
//  --> src/grammar.rs:5:1
//   |
// 5 | :
//   | ^ expected item

pub struct ErrReport<'r> {
    pub line: usize,
    pub span: Range<usize>,
    pub col: crossterm::style::Color,
    pub msg: &'r str,
    pub underline: char,
    pub col_text: bool,
}

use crossterm::style::Stylize;

fn colorize_string_part(
    original: &str,
    span: Range<usize>,
    col: crossterm::style::Color,
) -> String {
    let before = &original[..span.start];
    let after = &original[span.end..];
    let spanned = &original[span];
    let colored_spanned = spanned.with(col);
    let mut out = before.to_string();
    out.push_str(&colored_spanned.to_string());
    out.push_str(after);
    out
}

fn decorate_line(line: &str, reports: &[&ErrReport]) -> Vec<String> {
    // check for clashs.
    // right now each report is on its line.
    let mut line = line.to_string();
    let mut additionnal_lines = Vec::new();
    for r in reports {
        if r.col_text {
            line = colorize_string_part(&line, r.span.clone(), r.col);
        }
        let mut add_line = String::new();
        let mut squiglies = String::from(r.underline).repeat(r.span.len());
        squiglies.push(' ');
        if r.msg.len() > 200 {
            squiglies.push_str("Here");
        } else {
            squiglies.push_str(r.msg);
        }
        add_line.push_str(&" ".repeat(r.span.start));
        add_line.push_str(&squiglies.with(r.col).bold().to_string());
        additionnal_lines.push(add_line);
    }
    additionnal_lines.insert(0, line);
    additionnal_lines
}

fn prefix_w_line_number(
    l: usize,
    size: usize,
    strs: &[String],
    col: crossterm::style::Color,
    bold: bool,
) -> Vec<String> {
    let mut num_as_str = format!("{l}");
    let padding = " ".repeat(size - num_as_str.len());
    num_as_str.push_str(&padding);
    num_as_str.push(' ');
    let mut no_num_as_str = format!("{}| ", " ".repeat(num_as_str.len())).with(col);
    num_as_str.push_str("| ");
    let mut num_as_str = num_as_str.with(col);
    if bold {
        num_as_str = num_as_str.bold();
        no_num_as_str = no_num_as_str.bold();
    }
    let num_as_str = num_as_str.to_string();
    let no_num_as_str = no_num_as_str.to_string();
    let fst_line = &strs[0..1];
    let nxt_lines = &strs[1..];
    let mut out = Vec::new();
    for l in fst_line {
        out.push(format!("{}{}", num_as_str, l));
    }
    for l in nxt_lines {
        out.push(format!("{}{}", no_num_as_str, l));
    }
    out
}

pub fn create_report_content(
    lines: Range<usize>,
    reports: Vec<ErrReport>,
    onemore: bool,
) -> String {
    let mut s = String::new();
    let size = format!("{}", lines.end).len();
    s.push_str(&" ".repeat(size + 1));
    s.push_str(&"|".with(crossterm::style::Color::Blue).bold().to_string());
    s.push('\n');
    let om = s.clone();
    let lock = crate::GLOBAL.lock().unwrap();
    let cf = &lock.code_file.as_ref().unwrap();
    for i in lines {
        let line = cf.get_line(i);
        let reports_for_this_line: Vec<_> = reports.iter().filter(|r| r.line == i).collect();
        let mod_lines = decorate_line(line, &reports_for_this_line);
        let mod_lines =
            prefix_w_line_number(i, size, &mod_lines, crossterm::style::Color::Blue, true);
        for l in mod_lines {
            s.push_str(&l);
            s.push('\n');
        }
    }
    if onemore {
        s.push_str(&om);
    }
    s
}

pub fn report_message_header(
    loc: &Location,
    msg: &str,
    typ: &str,
    col: crossterm::style::Color,
    bold: bool,
) {
    let lock = crate::GLOBAL.lock().unwrap();
    let cfname = &lock.code_file.as_ref().unwrap().filename;
    let mut typ = typ.with(col);
    let mut remain = format!(": {}", msg).stylize();
    if bold {
        typ = typ.bold();
        remain = remain.bold();
    }
    println!(
        "{}{}\n{}{}:{}",
        typ,
        remain,
        "  --> ".with(crossterm::style::Color::Blue).bold(),
        cfname,
        loc
    );
}

pub fn report_error_ext_one_more(loc: &Location, text: &str, under_text: &str) {
    report_message_header(loc, text, "error", crossterm::style::Color::DarkRed, true);
    let report_content = create_report_content(
        loc.line..loc.line + 1,
        vec![ErrReport {
            line: loc.line,
            span: loc.span.clone(),
            col: crossterm::style::Color::DarkRed,
            msg: under_text,
            underline: '^',
            col_text: false,
        }],
        true,
    );
    print!("{}", report_content);
}

pub fn report_error_ext(loc: &Location, text: &str, under_text: &str) {
    report_message_header(loc, text, "error", crossterm::style::Color::DarkRed, true);
    let report_content = create_report_content(
        loc.line..loc.line + 2,
        vec![ErrReport {
            line: loc.line,
            span: loc.span.clone(),
            col: crossterm::style::Color::DarkRed,
            msg: under_text,
            underline: '^',
            col_text: false,
        }],
        false,
    );
    println!("{}", report_content);
}

pub fn report_error(loc: &Location, text: &str) {
    report_message_header(loc, text, "error", crossterm::style::Color::DarkRed, true);
    let report_content = create_report_content(
        loc.line..loc.line + 1,
        vec![ErrReport {
            line: loc.line,
            span: loc.span.clone(),
            col: crossterm::style::Color::DarkRed,
            msg: "",
            underline: '^',
            col_text: false,
        }],
        false,
    );
    println!("{}", report_content);
}

pub fn report_note_simple(text: &str) {
    println!(
        "{}{} {}",
        "note".bold().with(crossterm::style::Color::Blue),
        ":".bold(),
        text.bold()
    );
}
