use std::collections::VecDeque;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: u32 = s.trim().parse().unwrap();
    let mut q: VecDeque<u32> = (1..=s).collect();
    for _ in 1..s {
        q.pop_front();
        if let Some(n) = q.pop_front() {
            q.push_back(n);
        }
    }
    println!("{}", q.get(0).unwrap());
}