fn main() {
    let l = || std::io::stdin().lines().next().unwrap().unwrap();
    let (s, e, mut v) = (l(), l(), Vec::new());
    for c in s.chars() {
        v.push(c);
        if v.len() >= e.len() {
            let t: String = v.iter().skip(v.len() - e.len()).collect();
            if t == e {
                for _ in 0..e.len() {
                    v.pop();
                }
            }
        }
    }
    if v.is_empty() {
        println!("FRULA");
    } else {
        println!("{}", v.into_iter().collect::<String>());
    }
}