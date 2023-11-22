fn main() {
    let mut l = std::io::stdin().lines();
    let mut l = || l.next().unwrap().unwrap().parse().unwrap();
    let (n, mut o, mut s, mut c) = (l(), Vec::new(), Vec::new(), 1);
    for _ in 0..n {
        let i = l();
        while c <= i {
            o.push("+");
            s.push(c);
            c += 1;
        }
        if let Some(top) = s.pop() {
            if top != i {
                println!("NO");
                return;
            } else {
                o.push("-");
            }
        }
    }
    println!("{}", o.join("\n"));
}