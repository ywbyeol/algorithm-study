use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();
    let v: Vec<u32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let f: Vec<u32> = (1..=v[0]).filter(|&x| v[0] % x == 0).collect();
    if v[1] as usize > f.len() {
        println!("0");
    } else {
        println!("{}", f[v[1] as usize - 1]);
    }
}