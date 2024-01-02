use std::collections::HashSet;

fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut h = HashSet::from(["ChongChong"]);
    for l in s.lines().skip(1) {
        let (a, b) = l.split_once(" ").unwrap();
        if h.get(a).is_some() || h.get(b).is_some() {
            h.insert(a);
            h.insert(b);
        }
    }
    print!("{}", h.len());
}