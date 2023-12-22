use std::collections::VecDeque;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let (n, m) = (s.next().unwrap(), s.next().unwrap());
    let (mut q, mut c) = (VecDeque::from_iter(1..=n), 0);
    for l in (n - m + 1..=n).rev() {
        let t = s.next().unwrap();
        let p = q.iter().enumerate().find(|n| n.1 == &t).unwrap().0 as i32;
        c += if p <= l - p {
            q.rotate_left(p as usize);
            p
        } else {
            q.rotate_right((l - p) as usize);
            l - p
        };
        q.pop_front();
    }
    print!("{}", c);
}