fn main() {
    let u = || std::io::stdin().lines().next().unwrap().unwrap();
    let p = |n: char| n.to_digit(10).unwrap() as u8;
    let mut c = Vec::new();
    for (x, y) in u().chars().zip(u().chars()) {
        (c.push(p(x)), c.push(p(y)));
    }
    f(&c);
}

fn f(v: &[u8]) {
    if v.len() == 2 {
        println!("{}{}", v[0], v[1]);
        return;
    }
    let t: Vec<_> = v.windows(2).map(|w| (w[0] + w[1]) % 10).collect();
    f(&t)
}