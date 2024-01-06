fn main() {
    let s = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut s = s.split_whitespace().flat_map(str::parse);
    let mut r = || s.next().unwrap();
    let (n, q) = (r(), r());
    let li = Vec::from_iter((0..n).map(|_| r()));
    for _ in 0..q {
        let t = r();
        let mut r = li.iter().scan(0, |s, x| {
            *s += x;
            Some(*s)
        });
        println!("{}", r.position(|s| s > t).unwrap() + 1);
    }
}