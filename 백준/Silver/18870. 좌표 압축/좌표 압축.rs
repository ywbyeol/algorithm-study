use std::io::{self, Write};

fn main() {
    let t = io::read_to_string(io::stdin()).unwrap();
    let t = t.split_whitespace().map(|n| n.parse::<i32>().unwrap());
    let mut v = Vec::from_iter(t.skip(1).enumerate().map(|(i, s)| (i, s)));
    v.sort_by(|a, b| a.1.cmp(&b.1));
    let (mut t, mut p) = (0, v[0].1);
    for n in &mut v {
        if n.1 != p {
            t += 1;
        }
        p = n.1;
        n.1 = t;
    }
    v.sort_by(|a, b| a.0.cmp(&b.0));
    let mut o = io::BufWriter::new(io::stdout().lock());
    v.iter().for_each(|c| write!(o, "{} ", c.1).unwrap());
}