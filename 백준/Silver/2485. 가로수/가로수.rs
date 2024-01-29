fn main() {
    let i = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut i = i.split_whitespace().flat_map(str::parse);
    let mut r = || i.next().unwrap();
    let (n, mut p) = (r(), r());
    let v = Vec::from_iter((1..n).map(|_| {
        let d = r() - p;
        p += d;
        d
    }));
    let g = v.iter().fold(v[0], |a, &x| g(a, x));
    println!("{}", v.iter().fold(0, |a, &x| a + x / g - 1));
}

fn g(x: u32, y: u32) -> u32 {
    return if y == 0 { x } else { g(y, x % y) };
}