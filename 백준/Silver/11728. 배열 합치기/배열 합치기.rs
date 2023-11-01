use std::io::{self, Write};

fn main() {
    let l = || io::stdin().lines().next().unwrap().unwrap();
    let c = |s: String| s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let (_, a, b, mut p, mut q): (_, Vec<i32>, _, _, _) = (l(), c(l()), c(l()), 0, 0);
    let mut r = Vec::new();
    while p < a.len() && q < b.len() {
        if a[p] < b[q] {
            r.push(a[p]);
            p += 1;
        } else {
            r.push(b[q]);
            q += 1;
        }
    }
    (r.extend(&a[p..]), r.extend(&b[q..]));
    let mut o = io::BufWriter::new(io::stdout().lock());
    r.iter().for_each(|n| write!(o, "{} ", n).unwrap());
}