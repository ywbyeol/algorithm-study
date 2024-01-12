fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s: Vec<String> = Vec::from_iter(s.lines().skip(1).map(|w| w.chars().rev().collect()));
    for i in 1..=s[0].len() {
        let mut t = Vec::from_iter(s.iter().map(|w| &w[0..i]));
        (t.sort(), t.dedup());
        if t.len() == s.len() {
            print!("{}", i);
            break;
        }
    }
}