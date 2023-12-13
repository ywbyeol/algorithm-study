use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (s, mut h) = (s.split_whitespace().skip(1), HashMap::new());
    s.for_each(|l| *h.entry(l.split('.').nth(1).unwrap()).or_insert(0) += 1);
    let mut h = Vec::from_iter(h);
    let mut o = io::BufWriter::new(io::stdout());
    h.sort_by_key(|(n, _)| *n);
    for e in h {
        writeln!(o, "{} {}", e.0, e.1).unwrap();
    }
}