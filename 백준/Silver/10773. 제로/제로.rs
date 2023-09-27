use std::io::{self, Read};

fn main() {
    let mut i = String::new();
    io::stdin().read_to_string(&mut i).unwrap();
    let mut s = Vec::new();
    for n in i.lines().skip(1) {
        if n == "0" {
            s.pop().unwrap();
        } else {
            s.push(n.trim().parse().unwrap());
        }
    }
    println!("{}", s.iter().sum::<u32>());
}