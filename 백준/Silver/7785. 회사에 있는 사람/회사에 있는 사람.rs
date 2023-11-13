use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let mut s = HashSet::new();
    for l in io::stdin().lines().skip(1).flatten() {
        let (v, c) = l.split_once(' ').unwrap();
        if c.starts_with('l') {
            s.remove(v);
        } else {
            s.insert(v.to_owned());
        }
    }
    let mut s = Vec::from_iter(s);
    s.sort_by(|a, b| b.cmp(a));
    let mut o = io::BufWriter::new(io::stdout().lock());
    s.iter().for_each(|l| writeln!(o, "{}", l).unwrap());
}