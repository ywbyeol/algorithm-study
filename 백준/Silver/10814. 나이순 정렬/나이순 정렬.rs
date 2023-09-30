use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut m = Vec::new();
    for _ in 0..s.trim().parse().unwrap() {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let l: Vec<&str> = s.split_whitespace().collect();
        m.push((l[0].parse::<i32>().unwrap(), l[1].to_owned()));
    }
    m.sort_by_key(|k| k.0);
    let mut o = io::BufWriter::new(io::stdout().lock());
    for (x, y) in m {
        writeln!(o, "{} {}", x, y).unwrap();
    }
}