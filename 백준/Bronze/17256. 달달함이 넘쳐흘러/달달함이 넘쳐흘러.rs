fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<u16> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    println!("{} {} {}", v[3] - v[2], v[4] / v[1], v[5] - v[0]);
}