use std::io::{self, Write};

fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let (mut o, mut s) = (io::BufWriter::new(io::stdout().lock()), Vec::new());
    let mut f = |s: &dyn std::fmt::Display| writeln!(o, "{}", s).unwrap();
    for l in i.lines().skip(1) {
        let mut l = l.split(" ");
        match (l.next().unwrap(), l.next().unwrap_or("-1")) {
            ("push", e) => s.push(e),
            ("pop", e) => f(&s.pop().unwrap_or(e)),
            ("size", _) => f(&s.len()),
            ("empty", _) => f(&(s.is_empty() as u8)),
            ("top", e) => f(s.last().unwrap_or(&e)),
            _ => {}
        }
    }
}