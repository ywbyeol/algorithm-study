use std::cmp::Ordering::*;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<u16> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let a = match (v[3] * 3 + v[4] * 20 + v[5] * 120).cmp(&(v[0] * 3 + v[1] * 20 + v[2] * 120)) {
        Less => "Max",
        Equal => "Draw",
        Greater => "Mel",
    };
    println!("{}", a);
}