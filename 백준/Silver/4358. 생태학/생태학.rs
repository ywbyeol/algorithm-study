use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let (mut m, mut c) = (HashMap::new(), 0.0);
    for t in io::stdin().lines().flatten() {
        (*m.entry(t).or_insert(0.0) += 1.0, c += 1.0);
    }
    let (mut m, mut o) = (Vec::from_iter(m), io::BufWriter::new(io::stdout()));
    m.sort_unstable_by_key(|(k, _)| k.clone());
    for (k, v) in m {
        writeln!(o, "{} {:.4}", k, ((v / c) * 1000000f64).round() / 10000.0).unwrap()
    }
}