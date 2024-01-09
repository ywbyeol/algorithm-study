use std::collections::HashSet;

fn main() {
    let l = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut l = l.split_whitespace();
    let mut r = || l.next().unwrap().parse().unwrap();
    let (n, k, mut v, mut h) = (r(), r(), Vec::from_iter(l), HashSet::new());
    p(&mut h, &mut v, 0, n, k);
    print!("{}", h.len());
}

fn p(h: &mut HashSet<String>, v: &mut Vec<&str>, s: usize, n: usize, k: usize) {
    if s == k {
        h.insert(v[..k].join(""));
        return;
    }
    for i in s..n {
        v.swap(s, i);
        p(h, v, s + 1, n, k);
        v.swap(s, i);
    }
}