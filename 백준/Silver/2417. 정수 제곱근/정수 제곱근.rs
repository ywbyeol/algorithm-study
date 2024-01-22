fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let n: u64 = n.trim().parse().unwrap();
    let q = (n as f64).sqrt() as u64;
    print!("{}", if q * q >= n { q } else { q + 1 })
}