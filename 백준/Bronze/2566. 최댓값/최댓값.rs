use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let m = v.iter().max().unwrap();
    let i = v.iter().position(|&x| x == *m).unwrap();
    print!("{}\n{} {}", m, (i / 9) + 1, (i % 9) + 1);
}