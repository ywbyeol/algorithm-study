use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let s = io::read_to_string(io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (n, _) = (s.next().unwrap(), s.next());
    let a: HashSet<_> = s.clone().take(n).collect();
    let b = HashSet::from_iter(s.skip(n));
    let mut r = Vec::from_iter(a.difference(&b));
    r.sort_unstable();
    let mut o = io::BufWriter::new(io::stdout());
    writeln!(o, "{}", r.len()).unwrap();
    if !r.is_empty() {
        r.iter().for_each(|v| write!(o, "{} ", v).unwrap());
    }
}
