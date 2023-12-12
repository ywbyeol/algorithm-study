use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace();
    let mut r = || s.next().unwrap();
    let (n, _, mut h) = (r().parse().unwrap(), r(), HashMap::new());
    for _ in 0..n {
        h.insert(r(), r());
    }
    let mut o = io::BufWriter::new(io::stdout());
    s.for_each(|l| writeln!(o, "{}", h.get(l).unwrap()).unwrap());
}