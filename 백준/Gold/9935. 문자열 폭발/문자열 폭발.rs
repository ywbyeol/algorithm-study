fn main() {
    let l = || std::io::stdin().lines().next().unwrap().unwrap();
    let (s, e, mut v) = (l(), l(), Vec::new());
    for c in s.chars() {
        v.push(c);
        if v.ends_with(&Vec::from_iter(e.chars())) {
            v.drain(v.len() - e.len()..);
        }
    }
    if v.is_empty() {
        println!("FRULA");
    } else {
        println!("{}", String::from_iter(v.into_iter()));
    }
}