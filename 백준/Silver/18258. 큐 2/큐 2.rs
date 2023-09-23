use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut o = io::BufWriter::new(io::stdout().lock());
    let mut q: VecDeque<i32> = VecDeque::new();
    let n: i32 = s.trim().parse().unwrap();
    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let mut l = s.split_whitespace();
        let c = l.next().unwrap();
        let v: i32 = l.next().map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
        match (c, v) {
            ("push", e) => q.push_back(e),
            ("pop", _) => writeln!(o, "{}", q.pop_front().unwrap_or(-1)).unwrap(),
            ("size", _) => writeln!(o, "{}", q.len()).unwrap(),
            ("empty", _) => writeln!(o, "{}", q.is_empty() as u8).unwrap(),
            ("front", _) => writeln!(o, "{}", q.front().unwrap_or(&-1)).unwrap(),
            ("back", _) => writeln!(o, "{}", q.back().unwrap_or(&-1)).unwrap(),
            _ => unreachable!(),
        }
    }
}