use std::cmp::Ordering::*;
use std::io::{self, Write};

fn main() {
    let w = io::read_to_string(io::stdin()).unwrap().trim().to_string();
    let (l, mut r) = (w.len(), Vec::from_iter(w.bytes().map(Into::into)));
    let (mut s, mut t, mut p) = (Vec::from_iter(0..l), vec![0; l + 1], 1);
    (r.push(-1), t[l] = -1);
    while p < l {
        let c = r.clone();
        let f = |u: usize, v: usize| {
            if c[u] == c[v] {
                c[u + p].cmp(&c[v + p])
            } else {
                c[u].cmp(&c[v])
            }
        };
        s.sort_unstable_by(|&a, &b| f(a, b));
        t[s[0]] = 0;
        (0..l - 1).for_each(|i| t[s[i + 1]] = t[s[i]] + (f(s[i], s[i + 1]) == Less) as i32);
        std::mem::swap(&mut r, &mut t);
        p <<= 1;
    }
    let mut o = io::BufWriter::new(io::stdout());
    s.iter().for_each(|&i| writeln!(o, "{}", i).unwrap());
}