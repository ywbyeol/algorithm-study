fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    print!("{} ", s.trim().parse::<u32>().unwrap() * 4000);
}