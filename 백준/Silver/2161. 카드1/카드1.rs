use std::collections::VecDeque;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut q = VecDeque::from_iter(1..=s.trim().parse().unwrap());
    for _ in 0..q.len() {
        print!("{} ", q.pop_front().unwrap());
        if let Some(f) = q.pop_front() {
            q.push_back(f);
        } else {
            break;
        }
    }
}