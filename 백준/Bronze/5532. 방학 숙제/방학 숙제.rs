use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let v: Vec<f64> = s
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .take(5)
        .collect();
    println!(
        "{}",
        (v[0] as u16) - ((v[2] / v[4]).ceil() as u16).max((v[1] / v[3]).ceil() as u16)
    );
}