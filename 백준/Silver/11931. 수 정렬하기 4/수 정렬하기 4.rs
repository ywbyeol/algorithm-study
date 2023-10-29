use std::io::{self, Write};

fn main() {
    let (mut v, l) = (vec![false; 2000002], std::io::stdin().lines().skip(1));
    l.for_each(|n| v[(n.unwrap().trim().parse::<i32>().unwrap() + 1000000) as usize] = true);
    let mut o = io::BufWriter::new(io::stdout().lock());
    for i in (0..=2000000).rev() {
        if v[i] {
            writeln!(o, "{}", i as isize - 1000000).unwrap();
        }
    }
}