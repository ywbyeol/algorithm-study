use std::io::{self, Read, Write};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let mut s = Vec::new();
    for l in i.lines().skip(1) {
        let mut l = l.split_whitespace();
        let (c, v) = (l.next().unwrap(), l.next().unwrap_or("-1"));
        match (c, v) {
            ("1", e) => s.push(e),
            ("2", e) => writeln!(o, "{}", s.pop().unwrap_or(e)).unwrap(),
            ("3", _) => writeln!(o, "{}", s.len()).unwrap(),
            ("4", _) => writeln!(o, "{}", s.is_empty() as u8).unwrap(),
            ("5", e) => writeln!(o, "{}", s.last().unwrap_or(&e)).unwrap(),
            _ => unreachable!(),
        }
    }
}