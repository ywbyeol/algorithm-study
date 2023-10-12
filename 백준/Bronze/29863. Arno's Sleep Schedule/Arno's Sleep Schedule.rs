fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let v: Vec<u8> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
    let a = if v[0] > 19 {
        v[1] + 24 - v[0]
    } else {
        v[1] - v[0]
    };
    println!("{}", a);
}