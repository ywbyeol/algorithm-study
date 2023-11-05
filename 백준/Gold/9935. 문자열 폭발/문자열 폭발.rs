fn main() {
    let l = || std::io::stdin().lines().next().unwrap().unwrap();
    let (s, e, mut v) = (l(), l(), String::new());
    for c in s.chars() {
        v.push(c);
        if v.len() >= e.len() && &v[v.len() - e.len()..] == e {
            v.drain(v.len() - e.len()..);
        }
    }
    println!("{}", if v.is_empty() { "FRULA" } else { &v })
}