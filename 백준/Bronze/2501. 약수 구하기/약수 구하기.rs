use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<usize> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let f: usize = (1..=v[0])
        .filter(|&x| v[0] % x == 0)
        .nth(v[1] - 1)
        .unwrap_or(0);
    println!("{}", f);
}