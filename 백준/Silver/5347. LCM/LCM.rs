fn main() {
    for l in std::io::stdin().lines().skip(1) {
        let v = Vec::from_iter(l.unwrap().split(" ").map(|n| n.parse().unwrap()));
        println!("{}", v[0] / g(v[0], v[1]) * v[1]);
    }
}

fn g(x: u64, y: u64) -> u64 {
    return if y == 0 { x } else { g(y, x % y) };
}