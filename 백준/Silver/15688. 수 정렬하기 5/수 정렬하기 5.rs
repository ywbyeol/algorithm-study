use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};

fn main() {
    let i = io::stdin();
    let mut il = i.lock();
    let mut n = String::new();
    il.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut b = BTreeMap::new();
    for _ in 0..n {
        let mut s = String::new();
        il.read_line(&mut s).unwrap();
        let n: i32 = s.trim().parse().unwrap();
        *b.entry(n).or_insert(0) += 1;
    }
    let o = io::stdout();
    let mut ol = io::BufWriter::new(o.lock());
    for (n, c) in b {
        for _ in 0..c {
            write!(ol, "{}\n", n).unwrap();
        }
    }
}