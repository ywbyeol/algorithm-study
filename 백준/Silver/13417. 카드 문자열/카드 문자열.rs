fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace();
    let mut r = || i.next().unwrap();
    for _ in 0..r().trim().parse().unwrap() {
        let (n, mut d) = (r().parse().unwrap(), Vec::from([r()]));
        for _ in 1..n {
            let c = r();
            if c <= d[0] {
                d.insert(0, c);
            } else {
                d.push(c);
            }
        }
        println!("{}", String::from_iter(d));
    }
}