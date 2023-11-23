fn main() {
    let r = || std::io::stdin().lines().next().unwrap().unwrap();
    loop {
        let (mut l, mut h) = (0, 11);
        loop {
            let n = r().parse().unwrap();
            if n == 0 {
                return;
            }
            match r().chars().nth(4).unwrap() {
                'h' => h = if h == 0 { n } else { h.min(n) },
                'l' => l = l.max(n),
                _ => {
                    if n > l && n < h {
                        println!("Stan may be honest");
                    } else {
                        println!("Stan is dishonest");
                    };
                    break;
                }
            }
        }
    }
}