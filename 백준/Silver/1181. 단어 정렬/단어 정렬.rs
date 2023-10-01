use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse().unwrap();
    s.clear();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let mut v = Vec::with_capacity(n);
    s.lines().for_each(|w| v.push(w));
    v.sort_unstable_by(|a, b| {
        if a.len() != b.len() {
            a.len().cmp(&b.len())
        } else {
            a.cmp(&b)
        }
    });
    v.dedup();
    for w in v {
        println!("{}", w);
    }
}