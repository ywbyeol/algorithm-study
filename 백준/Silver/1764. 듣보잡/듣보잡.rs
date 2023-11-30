use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (s, mut m) = (s.split_whitespace().skip(2), HashMap::new());
    for v in s {
        let e = m.entry(v.to_string());
        e.and_modify(|v| *v = true).or_insert(false);
    }
    let mut m = Vec::from_iter(m.into_iter().filter(|v| v.1).map(|(a, _)| a));
    m.sort();
    let mut o = io::BufWriter::new(io::stdout());
    writeln!(o, "{}", m.len()).unwrap();
    m.into_iter().for_each(|v| writeln!(o, "{}", v).unwrap());
}