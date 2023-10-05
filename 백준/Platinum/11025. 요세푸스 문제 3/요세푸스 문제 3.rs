fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (n, k) = s.trim().split_once(' ').unwrap();
    let k: u32 = k.trim().parse().unwrap();
    let mut a = 0;
    for i in 2..=n.parse().unwrap() {
        a = (a + k) % i;
    }
    println!("{}", a + 1);
}