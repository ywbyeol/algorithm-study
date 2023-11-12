use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace();
    let mut r = || s.next().unwrap().to_owned();
    let (n, m): (usize, u32) = (r().parse().unwrap(), r().parse().unwrap());
    let v = Vec::from_iter((1..=n).map(|_| r().to_owned()));
    let h: HashMap<_, _> = (1..=n).map(|i| (v[i - 1].clone(), i)).collect();
    let mut o = io::BufWriter::new(io::stdout().lock());
    for _ in 0..m {
        let t = r();
        match t.parse::<usize>() {
            Ok(i) => writeln!(o, "{}", v[i - 1]).unwrap(),
            Err(_) => writeln!(o, "{}", h[&t]).unwrap(),
        }
    }
}