fn main() {
    std::io::stdin().lines().skip(1).flatten().for_each(f);
}

fn f(l: String) {
    let mut s = Vec::new();
    for c in l.chars() {
        match c {
            '(' => s.push(c),
            ')' if s.pop() != Some('(') => s.push(')'),
            _ => {}
        }
    }
    println!("{}", if s.is_empty() { "YES" } else { "NO" });
}