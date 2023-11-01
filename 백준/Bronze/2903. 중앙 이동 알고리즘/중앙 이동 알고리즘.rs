fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    print!("{}", (2_u32.pow(n.trim().parse().unwrap()) + 1).pow(2));
}