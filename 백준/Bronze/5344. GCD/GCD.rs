use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut v: Vec<i32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for c in v[1..].chunks_mut(2) {
        while c[1] != 0 {
            let temp = c[1];
            c[1] = c[0] % c[1];
            c[0] = temp;
        }
        println!("{}", c[0]);
    }
}