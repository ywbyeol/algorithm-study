use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Write};

fn main() {
    let (mut h, l) = (BinaryHeap::new(), io::stdin().lines().skip(1));
    let mut o = io::BufWriter::new(io::stdout().lock());
    for l in l.flat_map(|v| v.unwrap().parse()) {
        match l {
            0 => writeln!(o, "{}", h.pop().unwrap_or(A(0)).0).unwrap(),
            _ => h.push(A(l)),
        }
    }
}

#[derive(Eq, PartialEq)]
struct A(i32);

impl Ord for A {
    fn cmp(&self, o: &Self) -> Ordering {
        o.0.abs().cmp(&self.0.abs()).then_with(|| o.0.cmp(&self.0))
    }
}

impl PartialOrd for A {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}