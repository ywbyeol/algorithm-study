use std::collections::VecDeque;

fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let i = i.split_whitespace().skip(1).flat_map(str::parse::<i32>);
    let mut d = VecDeque::from_iter(i.enumerate());
    for _ in 1..d.len() {
        let p = d.pop_front().unwrap();
        print!("{} ", p.0 + 1);
        if p.1 > 0 {
            d.rotate_left((p.1 as usize - 1) % d.len());
        } else {
            d.rotate_right((p.1.unsigned_abs() as usize) % d.len())
        }
    }
    print!("{}", d.pop_front().unwrap().0 + 1);
}