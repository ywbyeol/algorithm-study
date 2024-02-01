fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let n = n.trim().parse().unwrap();
    let mut d = vec![0u64; 36];
    d[0] = 1;
    (1..=n).for_each(|i| (0..i).for_each(|j| d[i] += d[j] * d[i - j - 1]));
    println!("{}", d[n]);
}