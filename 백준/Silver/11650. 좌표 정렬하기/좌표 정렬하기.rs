use std::io::{self, Write};

fn main() {
    let t = io::stdin().lines().skip(1);
    let mut v = Vec::from_iter(t.map(|l| f(&l.unwrap())));
    v.sort();
    let mut o = io::BufWriter::new(io::stdout().lock());
    for c in v {
        writeln!(o, "{} {}", c.0, c.1).unwrap();
    }
}

fn f(l: &str) -> (i32, i32) {
    let i = l.split_once(" ").unwrap();
    (i.0.parse().unwrap(), i.1.parse().unwrap())
}