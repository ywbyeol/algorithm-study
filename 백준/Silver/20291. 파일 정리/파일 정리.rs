use std::collections::HashMap;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (s, mut h) = (s.split_whitespace().skip(1), HashMap::new());
    s.for_each(|l| *h.entry(l.split('.').nth(1).unwrap()).or_insert(0) += 1);
    let mut h = Vec::from_iter(h);
    h.sort_by_key(|(n, _)| *n);
    h.iter().for_each(|e| println!("{} {}", e.0, e.1));
}