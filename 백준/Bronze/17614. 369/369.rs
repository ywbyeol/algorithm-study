fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n: u32 = s.trim().parse().unwrap();
    let c = (3..=n)
        .flat_map(|x| x.to_string().chars().collect::<Vec<_>>())
        .filter(|&c| c == '3' || c == '6' || c == '9')
        .count();
    println!("{}", c);
}