use std::io::{self, Write};

fn main() {
    let t = io::stdin().lines().skip(1);
    let t = t.map(|l| Vec::from_iter(l.unwrap().split(" ").map(|n| n.parse::<i32>().unwrap())));
    let mut v = Vec::from_iter(t);
    v.sort_by(|a, b| a[1].cmp(&b[1]).then_with(|| a[0].cmp(&b[0])));
    let mut o = io::BufWriter::new(io::stdout().lock());
    for c in v {
        writeln!(o, "{} {}", c[0], c[1]).unwrap();
    }
}