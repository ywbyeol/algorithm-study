fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    println!("{}\n3", (n.trim().parse::<usize>().unwrap()).pow(3));
}