fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s = s.trim();
    let mut v = Vec::from_iter((0..s.len()).map(|i| &s[i..]));
    v.sort();
    println!("{}", v.join("\n"));
}