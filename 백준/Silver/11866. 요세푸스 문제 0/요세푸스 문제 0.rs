fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (n, k) = s.trim().split_once(' ').unwrap();
    let k: usize = k.trim().parse().unwrap();
    let mut v: Vec<_> = (1..=n.parse().unwrap()).collect();
    let mut a = Vec::new();
    let mut i = k - 1;
    while !v.is_empty() {
        i %= v.len();
        a.push(v.remove(i).to_string());
        i += k - 1;
    }
    println!("<{}>", a.join(", "));
}