use std::collections::HashSet;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let s = s.split_whitespace().skip(1);
    let mut set = HashSet::new();
    for w in s {
        let mut t = Vec::from_iter(w.chars());
        t.sort();
        set.insert(t);
    }
    print!("{}", set.len());
}