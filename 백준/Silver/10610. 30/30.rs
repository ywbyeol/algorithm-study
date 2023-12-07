fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = Vec::from_iter(s.chars().flat_map(|c| c.to_digit(10)));
    if !s.contains(&0) || s.iter().sum::<u32>() % 3 != 0 {
        print!("-1");
        return;
    }
    s.sort_by(|a, b| b.cmp(a));
    let s = Vec::from_iter(s.iter().map(|n| n.to_string())).join("");
    print!("{}", s);
}