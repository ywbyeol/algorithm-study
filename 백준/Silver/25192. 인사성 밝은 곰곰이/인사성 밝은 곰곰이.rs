use std::collections::HashSet;

fn main() {
    let (mut h, mut c) = (HashSet::new(), 0);
    for s in std::io::stdin().lines().skip(2).flatten() {
        if s == "ENTER" {
            (c += h.len(), h.clear());
            continue;
        }
        h.insert(s);
    }
    println!("{}", c + h.len());
}