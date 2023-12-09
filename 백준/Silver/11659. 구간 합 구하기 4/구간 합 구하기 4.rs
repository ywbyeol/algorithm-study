use std::io::{self, Write};

fn main() {
    let l = io::read_to_string(io::stdin()).unwrap();
    let mut l = l.split_whitespace().flat_map(str::parse);
    let mut r = || l.next().unwrap();
    let (n, m, mut v) = (r(), r(), vec![0, r() as i32]);
    (1..n).for_each(|i| v.push(v[i] + r() as i32));
    let mut o = io::BufWriter::new(io::stdout());
    (0..m).for_each(|_| writeln!(o, "{}", -v[r() - 1] + v[r()]).unwrap());
}