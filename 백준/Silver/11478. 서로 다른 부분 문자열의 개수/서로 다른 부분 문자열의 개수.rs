use std::collections::HashSet;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (mut h, v) = (HashSet::new(), Vec::from_iter(s.trim().chars()));
    (1..=s.len()).for_each(|i| h.extend(v.windows(i).map(|w| String::from_iter(w))));
    print!("{}", h.len());
}