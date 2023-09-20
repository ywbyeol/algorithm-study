use std::collections::HashMap;
use std::io::{self, Read, Write};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let l: Vec<&str> = i.lines().collect();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let mut m = HashMap::from([("F", 0.0)]);
    let mut t = 4.5;
    for e in ["A+", "A0", "B+", "B0", "C+", "C0", "D+", "D0"] {
        m.insert(e, t);
        t = t - 0.5;
    }
    let (mut p, mut s) = (0.0, 0.0);
    for l in l {
        let v: Vec<&str> = l.split_whitespace().collect();
        if v[2] != "P" {
            let a = v[1].parse::<f64>().unwrap();
            p = p + a * m.get(v[2]).unwrap();
            s = s + a;
        }
    }
    writeln!(o, "{}", p / s).unwrap();
}