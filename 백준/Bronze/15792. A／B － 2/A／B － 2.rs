use std::io::{self, Read, Write};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut o = io::BufWriter::new(io::stdout().lock());
    write!(o, "{}.", v[0] / v[1]).unwrap();
    for _ in 0..1000 {
        v[0] = (v[0] % v[1]) * 10;
        write!(o, "{}", v[0] / v[1]).unwrap();
        if v[0] == 0 {
            break;
        }
    }
}