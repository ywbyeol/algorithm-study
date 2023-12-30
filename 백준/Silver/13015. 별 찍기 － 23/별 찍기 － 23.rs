use std::io::{self, Write};

fn main() {
    let n = io::read_to_string(io::stdin()).unwrap();
    let (n, mut v) = (n.trim().parse().unwrap(), Vec::new());
    let (t, p) = ("*".repeat(n), format!("*{}*", " ".repeat(n - 2)));
    v.push(format!("{}{}{}", t, " ".repeat(n * 2 - 3), t));
    for i in 1..n - 1 {
        let (t, m) = (" ".repeat(i), " ".repeat((n - 1 - i) * 2 - 1));
        v.push(format!("{}{}{}{}{}", t, p, m, p, t));
    }
    v.push(format!("{}{}{}", " ".repeat(n - 1), p, p.split_at(1).1));
    let mut o = io::BufWriter::new(io::stdout());
    for l in v.iter().chain(v.iter().rev().skip(1)) {
        writeln!(o, "{}", l.trim_end()).unwrap();
    }
}