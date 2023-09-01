use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<u32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", (v[0] * v[4]).min(v[1] + if v[2] > v[4] { 0 } else { v[3] * (v[4] - v[2]) }));
}