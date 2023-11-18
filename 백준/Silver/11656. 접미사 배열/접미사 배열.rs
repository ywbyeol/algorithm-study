fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut v = Vec::from_iter((0..s.trim().len()).map(|i| s.trim().split_at(i).1));
    v.sort();
    v.iter().for_each(|a| println!("{}", a));
}