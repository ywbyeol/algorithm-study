use std::collections::BinaryHeap;
use std::io::{self, Write};

fn main() {
    let (mut h, l) = (BinaryHeap::new(), io::stdin().lines().skip(1));
    let mut o = io::BufWriter::new(io::stdout().lock());
    for l in l.flat_map(|v| v.unwrap().parse()) {
        match l {
            0 => writeln!(o, "{}", h.pop().unwrap_or(0)).unwrap(),
            _ => h.push(l),
        }
    }
}