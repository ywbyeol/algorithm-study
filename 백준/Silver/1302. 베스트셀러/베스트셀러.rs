use std::collections::HashMap;

fn main() {
    let mut b = HashMap::new();
    for n in std::io::stdin().lines().skip(1).flatten() {
        *b.entry(n).or_insert(0) += 1;
    }
    let m = b.values().max().cloned().unwrap();
    let mut b = Vec::from_iter(b.into_iter().filter(|t| t.1 == m).map(|t| t.0));
    b.sort_unstable();
    print!("{}", b[0]);
}