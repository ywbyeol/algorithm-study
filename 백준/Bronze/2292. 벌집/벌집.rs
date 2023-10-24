fn main() {
    let n = std::io::read_to_string(std::io::stdin()).unwrap();
    let c = ((n.trim().parse::<f64>().unwrap() - 1.0) / 3.0).sqrt() + 1.0;
    println!("{}", c.round());
}