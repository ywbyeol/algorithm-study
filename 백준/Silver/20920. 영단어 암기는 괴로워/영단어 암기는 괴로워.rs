use std::collections::BTreeMap;
use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace().skip(1);
    let (m, mut v) = (s.next().unwrap().parse().unwrap(), BTreeMap::new());
    for w in s.filter(|w| w.len() >= m) {
        v.entry(w).and_modify(|c| *c += 1).or_insert(1);
    }
    let mut m = Vec::from_iter(v);
    m.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.len().cmp(&a.0.len())));
    let mut o = io::BufWriter::new(io::stdout().lock());
    m.into_iter().for_each(|w| writeln!(o, "{}", w.0).unwrap());
}