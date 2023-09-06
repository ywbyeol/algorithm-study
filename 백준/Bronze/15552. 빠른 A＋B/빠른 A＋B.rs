use std::io::{self, Read, Write};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut o = io::BufWriter::new(io::stdout().lock());
    for i in (0..(v[0] * 2)).step_by(2) {
        writeln!(o, "{}", v[i + 1] + v[i + 2]).unwrap();
    }
}