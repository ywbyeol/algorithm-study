use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<f64> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", 1.0 / (1.0 + (10_f64.powf((v[1] - v[0]) / 400.0))));
}