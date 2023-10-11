fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v = s.split_whitespace().skip(1);
    let mut v: Vec<i16> = v.map(|n| n.trim().parse().unwrap()).collect();
    v.sort_unstable();
    v.dedup();
    v.iter().for_each(|n| print!("{} ", n));
}