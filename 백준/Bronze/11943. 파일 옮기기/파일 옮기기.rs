fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s: Vec<u16> = s.split_whitespace().map(|v| v.parse().unwrap()).collect();
    println!("{}", (s[0] + s[3]).min(s[1] + s[2]));
}