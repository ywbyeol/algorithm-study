fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("{}", s.trim().parse::<u16>().unwrap() / 11 * 10);
}