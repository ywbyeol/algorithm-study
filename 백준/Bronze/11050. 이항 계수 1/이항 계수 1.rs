fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: Vec<u64> = s.split(" ").map(|x| x.trim().parse().unwrap()).collect();
    let (n, k) = (s[0], s[1]);
    println!("{}", if n < k { 0 } else { f(n) / (f(k) * f(n - k)) });
}

fn f(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * f(n - 1)
}