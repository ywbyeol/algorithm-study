use std::cmp::Ordering::*;

fn main() {
    let mut i = std::io::stdin().lines();
    let mut f = || i.next().unwrap().unwrap();
    let n: usize = f().parse().unwrap();
    let mut s: Vec<usize> = f().split_whitespace().map(|a| a.parse().unwrap()).collect();
    s.sort_unstable();
    let (x, mut c, mut l, mut r) = (f().parse().unwrap(), 0, 0, n - 1);
    while l < r {
        match (s[l] + s[r]).cmp(&x) {
            Less => l += 1,
            Equal => {
                c += 1;
                l += 1;
                r -= 1;
            }
            Greater => r -= 1,
        }
    }
    println!("{}", c);
}