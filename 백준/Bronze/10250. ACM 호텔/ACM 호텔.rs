use std::io::{self, Read};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let s: Vec<u16> = i
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    for c in s.chunks(3) {
        if c[2] % c[0] == 0 {
            println!("{}{:02}", c[0], c[2] / c[0]);
        } else {
            println!("{}{:02}", c[2] % c[0], c[2] / c[0] + 1);
        }
    }
}