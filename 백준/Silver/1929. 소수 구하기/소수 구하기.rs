use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let mut o = io::BufWriter::new(io::stdout());
    for n in (s.next().unwrap()..=s.next().unwrap()).filter(f) {
        writeln!(o, "{}", n).unwrap();
    }
}

fn f(n: &i64) -> bool {
    match n {
        n if n <= &1 => false,
        n if n <= &3 => true,
        n if n % 2 == 0 || n % 3 == 0 => false,
        _ => (5..=(*n as f64).sqrt() as i64)
            .step_by(6)
            .all(|i| n % i != 0 && n % (i + 2) != 0),
    }
}