use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let v: Vec<u16> = s
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .take(6)
        .collect();
    let a = v[0] * 3 + v[1] * 2 + v[2] * 1;
    let b = v[3] * 3 + v[4] * 2 + v[5] * 1;
    if a > b {
        println!("A");
    } else if a == b {
        println!("T");
    } else {
        println!("B");
    }
}