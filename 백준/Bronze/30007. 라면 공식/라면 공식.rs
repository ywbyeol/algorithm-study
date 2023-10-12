fn main() {
    for l in std::io::stdin().lines().skip(1) {
        let v: Vec<u32> = l.unwrap().split(" ").map(|n| n.parse().unwrap()).collect();
        println!("{}", v[0] * (v[2] - 1) + v[1]);
    }
}