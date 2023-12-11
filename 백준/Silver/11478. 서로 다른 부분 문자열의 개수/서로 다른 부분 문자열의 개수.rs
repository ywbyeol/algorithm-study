use std::collections::HashSet;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (s, mut h) = (s.trim().to_owned(), HashSet::new());
    for i in 1..=s.len() {
        for j in 0..=s.len() - i {
            h.insert(&s[j..j + i]);
        }
    }
    print!("{}", h.len());
}