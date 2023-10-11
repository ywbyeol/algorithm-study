fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("{}", s.trim().chars().next().unwrap() as u32 - 44031);
}