use std::cmp::Ordering::*;
use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (n, _) = (s.next().unwrap(), s.next());
    let mut a = Vec::with_capacity(n as usize);
    (0..n).for_each(|_| a.push(s.next().unwrap()));
    a.sort_unstable();
    let mut o = io::BufWriter::new(io::stdout());
    for d in s {
        let r = match a.binary_search_by(|e| match e.cmp(&d) {
            Equal => Greater,
            o => o,
        }) {
            Ok(i) | Err(i) if a.get(i) == Some(&d) => i as i32,
            _ => -1,
        };
        writeln!(o, "{}", r).unwrap();
    }
}