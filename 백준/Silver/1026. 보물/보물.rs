use std::io::{self, BufRead};

fn main() {
    let (_, mut a, mut b) = (r(), r(), r());
    (a.sort_unstable(), b.sort_unstable_by(|x, y| y.cmp(x)));
    println!("{}", a.iter().zip(&b).map(|(&x, &y)| x * y).sum::<u32>());
}

fn r() -> Vec<u32> {
    let l = io::stdin().lock().lines().next().unwrap().unwrap();
    l.split(" ").map(|n| n.parse().unwrap()).collect()
}