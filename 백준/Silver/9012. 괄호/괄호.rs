fn main() {
    std::io::stdin().lines().skip(1).for_each(|l| f(l.unwrap()));
}

fn f(l: String) {
    let mut s = Vec::new();
    for c in l.chars() {
        if let Some(&'(') = s.last() {
            if c == ')' {
                s.pop();
                continue;
            }
        }
        s.push(c);
    }
    println!("{}", if s.is_empty() { "YES" } else { "NO" });
}