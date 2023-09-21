use std::collections::HashSet;
use std::io::{self, Read, Write};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let l: Vec<&str> = i.lines().collect();
    let b: HashSet<isize> = f(l[1]).into_iter().collect();
    let d = f(l[3]);
    for n in d {
        write!(o, "{} ", b.contains(&n) as u8).unwrap();
    }
}

fn f(l: &str) -> Vec<isize> {
    return l.split_whitespace().map(|x| x.parse().unwrap()).collect();
}