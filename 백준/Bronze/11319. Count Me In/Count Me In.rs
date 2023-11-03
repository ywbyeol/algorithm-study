fn main() {
    for s in std::io::stdin().lines().skip(1).flatten() {
        let s = s.chars().filter(|&c| c.is_alphabetic());
        let l = s.clone().filter(|&c| !"AEIOUaeiou".contains(c)).count();
        println!("{} {}", l, s.count() - l);
    }
}