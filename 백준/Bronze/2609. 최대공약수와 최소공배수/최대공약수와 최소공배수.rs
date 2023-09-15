use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let p = &v[0] * &v[1];
    while v[1] != 0 {
        let t = v[1];
        v[1] = v[0] % v[1];
        v[0] = t;
    }
    println!("{}\n{}", v[0], p / v[0]);
}