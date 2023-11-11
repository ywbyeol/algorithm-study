use std::io::{self, Write};

fn main() {
    let l = io::read_to_string(io::stdin()).unwrap();
    let l: Vec<_> = l.split_whitespace().skip(1).flat_map(str::parse).collect();
    let (mut a, mut s) = (vec![-1; l.len()], vec![0]);
    for (i, &v) in l.iter().enumerate() {
        while let Some(&t) = s.last() {
            if l[t] < v {
                a[s.pop().unwrap()] = v;
            } else {
                break;
            }
        }
        s.push(i);
    }
    let mut o = io::BufWriter::new(io::stdout().lock());
    a.iter().for_each(|n| write!(o, "{} ", n).unwrap());
}