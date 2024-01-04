fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    print!("{}", s.lines().skip(1).filter(f).count());
}

fn f(l: &&str) -> bool {
    let mut s = Vec::new();
    for c in l.chars() {
        if s.last() == Some(&c) {
            s.pop();
        } else {
            s.push(c);
        }
    }
    s.is_empty()
}