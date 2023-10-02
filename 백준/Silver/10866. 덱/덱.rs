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
        let v: i32 = l.next().unwrap_or("0").parse().unwrap();
        match c {
            "push_front" => q.push_front(v),
            "push_back" => q.push_back(v),
            "pop_front" => writeln!(o, "{}", q.pop_front().unwrap_or(-1)).unwrap(),
            "pop_back" => writeln!(o, "{}", q.pop_back().unwrap_or(-1)).unwrap(),
            "size" => writeln!(o, "{}", q.len()).unwrap(),
            "empty" => writeln!(o, "{}", q.is_empty() as u8).unwrap(),
            "front" => writeln!(o, "{}", q.front().unwrap_or(&-1)).unwrap(),
            "back" => writeln!(o, "{}", q.back().unwrap_or(&-1)).unwrap(),
            _ => unreachable!(),
        }
    }
}