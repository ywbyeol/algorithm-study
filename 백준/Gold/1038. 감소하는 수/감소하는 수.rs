use std::collections::VecDeque;

fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let (n, mut a) = (n.trim().parse().unwrap(), 0);
    let mut d = VecDeque::from_iter(0_i64..if n > 1023 { 0 } else { 10 });
    while let Some(c) = d.pop_front() {
        a += 1;
        if a > n {
            println!("{}", c);
            return;
        }
        (0..(c % 10)).for_each(|i| d.push_back(c * 10 + i));
    }
    println!("-1");
}