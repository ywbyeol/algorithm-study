fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = Vec::from_iter(s.split_whitespace().flat_map(str::parse::<i32>).enumerate());
    s.sort_by_key(|t| t.1);
    let s = s.iter().skip(3);
    println!("{}", s.clone().map(|t| t.1).sum::<i32>());
    let mut s = Vec::from_iter(s.map(|t| t.0 + 1));
    s.sort();
    s.iter().for_each(|i| print!("{} ", i));
}