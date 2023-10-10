fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let (i, mut c) = (s.split_whitespace().skip(1), 0);
    i.for_each(|w| c += g(w));
    println!("{}", c);
}

fn g(w: &str) -> u8 {
    let (mut c, mut v) = (w.chars(), Vec::new());
    let mut p = c.next();
    while let Some(c) = c.next() {
        if Some(c) != p {
            if v.contains(&c) {
                return 0;
            } else {
                v.push(p.unwrap());
                p = Some(c);
            }
        }
    }
    1
}