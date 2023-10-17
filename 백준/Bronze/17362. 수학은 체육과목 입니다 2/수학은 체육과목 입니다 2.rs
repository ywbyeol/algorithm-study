fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let r = s.trim().parse::<u32>().unwrap() % 8;
    println!("{}", if r > 5 || r == 0 { (10 - r) % 8 } else { r });
}