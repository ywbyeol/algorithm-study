fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let a = char::from_u32(s.trim().parse::<u32>().unwrap() + 44031).unwrap();
    println!("{}", a);
}