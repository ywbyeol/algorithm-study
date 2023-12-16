use std::collections::HashMap;

fn main() {
    let mut l = std::io::stdin().lines().flatten();
    for _ in 0..l.next().unwrap().parse().unwrap() {
        l.next();
        let mut f = || Vec::from_iter(l.next().unwrap().split_whitespace().map(|v| v.to_owned()));
        let (a, b, mut c) = (f(), f(), f());
        let q = |j| a.iter().position(|x| x == j).unwrap();
        let d: HashMap<_, _> = b.iter().map(|j| (q(j), c.remove(0))).collect();
        let mut d = Vec::from_iter(d);
        d.sort_unstable_by_key(|a| a.0);
        d.iter().for_each(|(_, w)| print!("{} ", w));
        println!();
    }
}