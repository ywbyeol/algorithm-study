use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn main() {
    let r = || io::stdin().lock().lines().next().unwrap().unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    for _ in 0..r().parse().unwrap() {
        let mut m = BTreeMap::new();
        for _ in 0..r().parse().unwrap() {
            let t = r();
            let l = t.split_at(1);
            match (l.0, l.1.trim().parse().unwrap()) {
                ("I", n) => *m.entry(n).or_default() += 1,
                ("D", n) => f(&mut m, n),
                _ => unreachable!(),
            }
        }
        if m.is_empty() {
            writeln!(o, "EMPTY").unwrap();
        } else {
            let (b, f) = (*m.keys().last().unwrap(), *m.keys().next().unwrap());
            writeln!(o, "{} {}", b, f).unwrap();
        }
    }
}

fn f(m: &mut BTreeMap<i32, u32>, n: i32) {
    if m.is_empty() {
        return;
    }
    let n = match n {
        1 => *m.keys().last().unwrap(),
        _ => *m.keys().next().unwrap(),
    };
    let c = m.get_mut(&n).unwrap();
    *c -= 1;
    if *c == 0 {
        m.remove(&n);
    }
}