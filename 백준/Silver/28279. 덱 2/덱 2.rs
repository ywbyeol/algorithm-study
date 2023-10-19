use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let i = io::read_to_string(io::stdin()).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let mut f = |v: &str| writeln!(o, "{}", v).unwrap();
    let mut d = VecDeque::new();
    for mut l in i.lines().skip(1).map(|i| i.split_whitespace()) {
        match (l.next().unwrap(), l.next().unwrap_or("")) {
            ("1", e) => d.push_front(e),
            ("2", e) => d.push_back(e),
            ("3", _) => f(d.pop_front().unwrap_or("-1")),
            ("4", _) => f(d.pop_back().unwrap_or("-1")),
            ("5", _) => f(&d.len().to_string()),
            ("6", _) => f(&(d.is_empty() as u8).to_string()),
            ("7", _) => f(d.front().unwrap_or(&"-1")),
            ("8", _) => f(d.back().unwrap_or(&"-1")),
            _ => unreachable!(),
        }
    }
}