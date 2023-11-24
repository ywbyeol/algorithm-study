use std::io::{self, Write};

fn main() {
    let mut l = io::stdin().lines().flatten().peekable();
    let mut o = io::BufWriter::new(io::stdout().lock());
    while let Some(pw) = l.next() {
        if l.peek().is_some() {
            let r = if f(&pw) { "" } else { "not " };
            let _ = writeln!(o, "<{}> is {}acceptable.", pw, r);
        }
    }
}

fn f(l: &str) -> bool {
    let (v, mut c, mut s, mut h, mut p) = ("aeiou", 0, 0, false, ' ');
    for i in l.chars() {
        if v.contains(i) {
            (s += 1, c = 0, h = true);
        } else {
            (s = 0, c += 1);
        }
        if (s > 2 || c > 2) || (p == i && i != 'e' && i != 'o') {
            return false;
        }
        p = i;
    }
    h
}