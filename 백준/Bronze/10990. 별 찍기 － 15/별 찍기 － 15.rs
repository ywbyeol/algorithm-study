use std::io::{self, Write};
fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let s: usize = s.trim().parse().unwrap();
    writeln!(o, "{}*", " ".repeat(s - 1)).unwrap();
    for i in 0..(s - 1) {
        writeln!(o, "{}{}{}{}", " ".repeat(s - i - 2), "*", " ".repeat(i * 2 + 1), "*").unwrap();
    }
}