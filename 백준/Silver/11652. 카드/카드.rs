use std::{collections::HashMap, io};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut h = HashMap::new();
    for n in s.split_whitespace().skip(1).flat_map(str::parse::<i64>) {
        *h.entry(n).or_insert(0) += 1;
    }
    let m = h.iter().max_by(|k, v| k.1.cmp(v.1).then(v.0.cmp(k.0)));
    print!("{}", m.unwrap().0);
}