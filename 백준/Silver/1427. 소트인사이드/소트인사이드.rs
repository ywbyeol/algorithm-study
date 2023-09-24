use std::io::{self, Read, Write};

fn main() {
    let mut i = io::stdin().lock();
    let mut o = io::stdout().lock();
    let mut s = Vec::new();
    i.read_to_end(&mut s).unwrap();
    s.sort_unstable_by(|a, b| b.cmp(a));
    o.write_all(&s).unwrap();
}