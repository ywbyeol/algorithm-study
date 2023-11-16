fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let c = (1..=n.trim().parse().unwrap()).filter(|&i| i < 100 || i / 100 * 21 + i == i / 10 * 12);
    print!("{}", c.count());
}