use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let s = s.split_whitespace().skip(1).flat_map(str::parse);
    let mut o = io::BufWriter::new(io::stdout());
    s.for_each(|n| writeln!(o, "{}", (n..).find(|n| f(*n)).unwrap()).unwrap());
}

fn f(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    (2..=(n as f64).sqrt() as u64).all(|i| n % i != 0)
}