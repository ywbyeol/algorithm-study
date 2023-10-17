use std::io;

fn main() {
    io::stdin().lines().skip(1).for_each(|l| f(&l.unwrap()));
}

fn f(s: &str) {
    let v: Vec<u64> = s.split(" ").skip(1).map(|n| n.parse().unwrap()).collect();
    let mut c = 0;
    for i in 0..v.len() {
        ((i + 1)..v.len()).for_each(|j| c += g(v[i], v[j]));
    }
    println!("{}", c);
}

fn g(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        g(y, x % y)
    }
}