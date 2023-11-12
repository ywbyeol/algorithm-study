use std::fmt::Write;

fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let (i, mut s) = (i.split_once(' ').unwrap(), Vec::new());
    let (n, mut r) = (i.0.parse().unwrap(), String::new());
    f(n, i.1.trim().parse().unwrap(), &mut s, &mut r);
    println!("{}", r);
}

fn f(n: u8, m: u8, s: &mut Vec<u8>, r: &mut String) {
    if s.len() == m as usize {
        let l: Vec<_> = s.iter().map(|&x| x.to_string()).collect();
        let _ = writeln!(r, "{}", l.join(" "));
        return;
    }

    for i in 1..=n {
        if !s.contains(&i) {
            s.push(i);
            f(n, m, s, r);
            s.pop();
        }
    }
}