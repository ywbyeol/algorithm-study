fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let (n, mut m) = (n.trim().parse().unwrap(), [1u64; 116]);
    (3..n).for_each(|i| m[i] = m[i - 3] + m[i - 1]);
    print!("{}", m[n - 1]);
}