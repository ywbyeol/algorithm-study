fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let (r, c) = f(&s.trim(), 0);
    println!("{}\n{}", r, c);
}

fn f(n: &str, mut c: i32) -> (i32, String) {
    if n.len() > 1 {
        c += 1;
    }
    let s: i32 = n.chars().map(|x| x.to_digit(10).unwrap() as i32).sum();
    let r = s.to_string();
    match (r.len(), s % 3) {
        (1, 0) => (c, "YES".to_owned()),
        (1, _) => (c, "NO".to_owned()),
        (_, _) => f(&r, c),
    }
}