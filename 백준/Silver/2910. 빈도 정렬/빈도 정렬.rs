use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let (mut c, mut a) = (HashMap::new(), HashMap::new());
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let n = s.split_whitespace().skip(2).flat_map(str::parse::<i32>);
    for (i, n) in n.enumerate() {
        (*c.entry(n).or_insert(0) += 1, *a.entry(n).or_insert(i) += 1);
    }
    let (mut c, mut o) = (Vec::from_iter(c), io::BufWriter::new(io::stdout()));
    c.sort_unstable_by(|x, y| y.1.cmp(&x.1).then(a[&x.0].cmp(&a[&y.0])));
    for &(k, v) in c.iter() {
        (0..v).for_each(|_| write!(o, "{} ", k).unwrap());
    }
}