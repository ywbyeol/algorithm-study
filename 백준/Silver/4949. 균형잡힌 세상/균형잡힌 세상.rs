use std::io::{self, Write};

fn main() {
    let mut l = std::io::stdin().lines().peekable();
    let mut o = io::BufWriter::new(io::stdout().lock());
    while let Some(s) = l.next() {
        if l.peek().is_none() {
            continue;
        }
        writeln!(o, "{}", f(&s.unwrap())).unwrap();
    }
}

fn f(l: &str) -> &str {
    let mut s = Vec::new();
    for c in l.chars() {
        match c {
            '(' | '[' => s.push(c),
            ')' if s.pop() != Some('(') => return "no",
            ']' if s.pop() != Some('[') => return "no",
            _ => {}
        }
    }
    return if s.is_empty() { "yes" } else { "no" };
}