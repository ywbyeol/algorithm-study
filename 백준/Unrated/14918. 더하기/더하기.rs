fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s: i32 = s.split(" ").flat_map(|v| v.trim().parse::<i32>()).sum();
    print!("{}", s)
}