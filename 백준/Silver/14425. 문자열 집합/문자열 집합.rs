use std::collections::HashSet;

fn main() {
    let mut l = std::io::stdin().lines().flatten();
    let n = l.next().unwrap();
    let n = n.split_once(" ").unwrap().0.parse().unwrap();
    let s: HashSet<_> = l.by_ref().take(n).collect();
    println!("{}", l.filter(|w| s.contains(w)).count());
}