use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let r = || io::stdin().lines().flatten().next().unwrap();
    let n = r().parse().unwrap();
    let (mut m, h) = (HashMap::with_capacity(n), r());
    let h = h.split_whitespace().flat_map(str::parse::<i32>);
    for c in h {
        m.entry(c).and_modify(|v| *v += 1_u32).or_insert(1);
    }
    let (_, q) = (r(), r());
    let q = Vec::from_iter(q.split_whitespace().flat_map(str::parse));
    let mut o = io::BufWriter::new(io::stdout().lock());
    for k in q {
        write!(o, "{} ", m.get(&k).unwrap_or(&0)).unwrap()
    }
}