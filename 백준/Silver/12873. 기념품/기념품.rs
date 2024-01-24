use std::collections::VecDeque;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut q = VecDeque::from_iter(1..=s.trim().parse().unwrap());
    for i in 1..q.len() {
        q.rotate_left((i.pow(3) - 1) % q.len());
        q.pop_front().unwrap();
    }
    println!("{}", q[0]);
}